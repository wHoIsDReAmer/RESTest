# RESTest CLI
[![License](https://img.shields.io/badge/license-MIT-blue)](https://github.com/restest-cli/restest-cli/blob/main/LICENSE)

A simple and intuitive CLI tool for writing and executing HTTP API tests using a DSL (Domain Specific Language)

## ✨ Features
- Write HTTP API test cases with intuitive syntax
- Support various request options including HTTP methods, headers, body, and query parameters  
- Response status code and body validation
- Configurable timeouts

## 📦 Installation (not yet)
```
cargo install restest-cli
```

## 📖 Usage

### Writing Test Files
Create a `.rtest` file:
```
test "Get User Info"
endpoint "https://api.example.com/users/123"
method GET
timeout 1000
headers
    Authorization "Bearer your-token-here"
    Content-Type "application/json"
expect
    status 200
    body contains "\"username\": \"john\""

test "Create New User" 
endpoint "https://api.example.com/users"
method POST
headers
    Content-Type "application/json"
body "{\"username\": \"john\", \"email\": \"john@example.com\"}"
expect
    status 201
    body equals "{\"message\": \"User created successfully\"}"
```

### Running Tests
```
restest test --verbose
```

## 📚 Syntax

### Basic Structure
- test: Define a test case (required)
- endpoint: API endpoint URL (required)  
- method: HTTP method (GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD)
- timeout: Request timeout in milliseconds
- headers: Define HTTP headers
- body: Request body
- expect: Response validation rules

### Response Validation
- status: Verify HTTP status code
- body equals: Exact body match
- body contains: Check if body contains string

## 🔮 TODO Features
- [ ] JSON schema validation
- [ ] Variables and environment configuration
- [ ] Test result metrics
- [ ] Make applicable to CI/CD pipeline
- [ ] Support multiple test execution

## 🤝 Contributing
Bug reports, feature suggestions and pull requests are welcome!