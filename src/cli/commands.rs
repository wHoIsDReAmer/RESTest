use crate::cli::CliError;
use crate::dsl::ast::{ASTNode, BodyExpectation, ExpectNode, TestDefinition};
use crate::dsl::lexer::Lexer;
use crate::dsl::parser::core::Parser as DslParser;
use crate::dsl::tokens::HttpMethod;
use crate::request::simple_requester::SimpleRequester;
use crate::request::{RequestConfig, Requester};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

#[derive(Parser)]
pub(crate) enum Commands {
    /// Initialize
    Init {
        #[arg(short, long)]
        name: String,
    },
    /// Running tests in project
    Test {
        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },
}

impl Commands {
    pub async fn run(&self, directory: &str) -> Result<(), CliError> {
        match self {
            Commands::Init { name } => Self::handle_init(directory, name),
            Commands::Test { verbose } => Self::handle_test(directory, *verbose).await,
        }
    }

    fn handle_init(directory: &str, name: &str) -> Result<(), CliError> {
        let dir_path = Path::new(directory);
        if !dir_path.exists() {
            fs::create_dir_all(dir_path)?;
        }

        let target_path = dir_path.join(format!("{name}.rtest"));
        if target_path.exists() {
            return Err(CliError::AlreadyExists(target_path.display().to_string()));
        }

        let mut file = fs::File::create(&target_path)?;
        file.write_all(Self::sample_test_file().as_bytes())?;

        println!("Created template at {}", target_path.display());
        Ok(())
    }

    async fn handle_test(directory: &str, verbose: bool) -> Result<(), CliError> {
        let dir_path = Path::new(directory);
        let entries = fs::read_dir(dir_path)?;

        let mut found = 0usize;
        let mut passed = 0usize;
        let requester = SimpleRequester {};

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("rtest") {
                continue;
            }

            let content = fs::read_to_string(&path)?;
            let tokens = Lexer::string_to_tokens(&content)?;
            let mut parser = DslParser::new(tokens);
            let test_file = parser.parse()?;

            if test_file.tests.is_empty() {
                println!("{}: no tests parsed", path.display());
                continue;
            }

            println!("{}:", path.display());
            for test in test_file.tests {
                match test {
                    ASTNode::TestDefinition(name, test_def) => {
                        found += 1;
                        let result = Self::execute_test(&requester, &test_def).await;
                        match result {
                            Ok(outcome) => {
                                if outcome.passed {
                                    passed += 1;
                                    println!("  ✅ {}", name);
                                } else {
                                    println!("  ❌ {}", name);
                                    for fail in outcome.failures {
                                        println!("     - {}", fail);
                                    }
                                }
                                if verbose {
                                    println!("     status: {}", outcome.status);
                                }
                            }
                            Err(err) => {
                                println!("  ❌ {} (error: {err})", name);
                            }
                        }
                    }
                }
            }
        }

        if found == 0 {
            return Err(CliError::NoTestsFound(directory.to_string()));
        }

        println!("Summary: {passed}/{found} passed");
        Ok(())
    }

    fn sample_test_file() -> &'static str {
        r#"test "Sample Request"
endpoint "https://httpbin.org/get"
method GET
headers
  User-Agent "restest/0.1"
  Accept "application/json"
expect
  status 200
  body contains """url"": ""https://httpbin.org/get"""
"#
    }

    async fn execute_test(
        requester: &dyn Requester,
        test_def: &TestDefinition,
    ) -> Result<TestOutcome, CliError> {
        let method = match test_def.method {
            HttpMethod::None | HttpMethod::Unknown => HttpMethod::Get,
            m => m,
        };

        let mut headers = HashMap::new();
        for h in &test_def.headers {
            headers.insert(h.key.clone(), h.value.clone());
        }

        let mut config = RequestConfig::new().with_headers(headers);

        if let Some(body) = &test_def.body {
            config = config.with_body(body.clone());
        }

        if let Some(timeout) = test_def.timeout {
            config = config.with_timeout(Duration::from_millis(timeout as u64));
        }

        let response = requester
            .send_request_with_config(test_def.endpoint.clone(), method, config)
            .await?;

        let mut failures = Vec::new();

        for expect in &test_def.expect {
            match expect {
                ExpectNode::Status(code) => {
                    if response.status != *code {
                        failures.push(format!("expected status {}, got {}", code, response.status));
                    }
                }
                ExpectNode::Body(BodyExpectation::Contains(fragment)) => {
                    if !response.raw_body.contains(fragment) {
                        failures.push(format!("body missing fragment: {fragment}"));
                    }
                }
                ExpectNode::Body(BodyExpectation::Equals(body)) => {
                    if &response.raw_body != body {
                        failures.push("body not equal".to_string());
                    }
                }
            }
        }

        Ok(TestOutcome {
            passed: failures.is_empty(),
            failures,
            status: response.status,
        })
    }
}

struct TestOutcome {
    passed: bool,
    failures: Vec<String>,
    status: u16,
}
