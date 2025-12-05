# Contributing to Open-Jarvis

Thank you for your interest in contributing to Open-Jarvis! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful, harassment-free environment for everyone, regardless of background or identity.

## Getting Started

### Prerequisites

1. Rust 1.75+ installed via [rustup](https://rustup.rs/)
2. Node.js 20+ and pnpm/npm
3. Git for version control
4. Familiarity with Tauri, Rust, and TypeScript/JavaScript

### Development Setup

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/Open-Jarvis.git
   cd Open-Jarvis
   ```

2. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Install dependencies and build:**
   ```bash
   cd src-tauri
   cargo build
   ```

4. **Run the application:**
   ```bash
   cargo run
   ```

## Development Workflow

### Code Style

#### Rust Code

- Follow the [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Use `rustfmt` for automatic formatting:
  ```bash
  cargo fmt
  ```
- Run `clippy` to catch common mistakes:
  ```bash
  cargo clippy -- -D warnings
  ```

**Rust conventions:**
- Use snake_case for functions and variables
- Use PascalCase for types and traits
- Keep functions focused and under 50 lines when possible
- Add documentation comments (`///`) for public APIs
- Use `Result<T, E>` for error handling
- Avoid `unwrap()` in production code; use proper error handling

#### Frontend Code

- Use consistent indentation (2 spaces)
- Follow JavaScript/TypeScript best practices
- Use meaningful variable and function names
- Add comments for complex logic

### Git Workflow

1. **Keep commits atomic** - One logical change per commit
2. **Write meaningful commit messages:**
   ```
   feat: Add voice command processing
   
   - Implement voice recognition module
   - Add audio input handling
   - Integrate with command parser
   ```

3. **Commit message format:**
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation changes
   - `style:` - Code style changes (formatting, etc.)
   - `refactor:` - Code refactoring
   - `test:` - Adding or updating tests
   - `chore:` - Maintenance tasks

### Testing Requirements

#### Unit Tests

- Write unit tests for new Rust functions
- Place tests in the same file or in a `tests` module:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      
      #[test]
      fn test_my_function() {
          assert_eq!(my_function(input), expected);
      }
  }
  ```

#### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

#### Integration Tests

- Add integration tests in `src-tauri/tests/` directory
- Test command handlers and Tauri-specific functionality

### Pull Request Process

1. **Before submitting:**
   - Ensure all tests pass: `cargo test`
   - Format code: `cargo fmt`
   - Check for linting issues: `cargo clippy -- -D warnings`
   - Update documentation if needed
   - Test on your target platform

2. **Creating a Pull Request:**
   - Use a clear, descriptive title
   - Reference any related issues: "Fixes #123"
   - Provide a detailed description of changes
   - Include screenshots for UI changes
   - List any breaking changes

3. **PR Template:**
   ```markdown
   ## Description
   Brief description of changes
   
   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update
   
   ## Testing
   - [ ] Unit tests added/updated
   - [ ] Integration tests added/updated
   - [ ] Manual testing performed
   
   ## Checklist
   - [ ] Code follows style guidelines
   - [ ] Self-review completed
   - [ ] Comments added for complex code
   - [ ] Documentation updated
   - [ ] No new warnings generated
   ```

4. **Review process:**
   - Maintainers will review your PR
   - Address feedback promptly
   - Keep the PR scope focused
   - Squash commits if requested

### Code Review Guidelines

When reviewing others' code:
- Be respectful and constructive
- Focus on the code, not the person
- Explain the "why" behind suggestions
- Approve when ready, or request changes with clear guidance

## Project Structure

```
Open-Jarvis/
├── src/                    # Frontend code (HTML, CSS, JS)
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   └── mcp/           # MCP integration modules
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── docs/                   # Documentation
├── .github/               # GitHub workflows and templates
└── README.md
```

## Documentation

- Update README.md for user-facing features
- Add/update docs/ for detailed documentation
- Include inline code comments for complex logic
- Use Rust doc comments (`///`) for public APIs:
  ```rust
  /// Processes a voice command and returns the result
  ///
  /// # Arguments
  ///
  /// * `command` - The command string to process
  ///
  /// # Returns
  ///
  /// Result containing the command output or an error
  pub fn process_command(command: &str) -> Result<String, Error> {
      // Implementation
  }
  ```

## Reporting Bugs

Use GitHub Issues with the bug report template:

1. Clear, descriptive title
2. Steps to reproduce
3. Expected behavior
4. Actual behavior
5. System information (OS, Rust version, etc.)
6. Screenshots if applicable

## Feature Requests

Use GitHub Issues with the feature request template:

1. Clear description of the feature
2. Use case and motivation
3. Possible implementation approach
4. Alternative solutions considered

## Security Vulnerabilities

**Do not open public issues for security vulnerabilities.**

Instead, email security concerns to: [sunil.kumar@example.com]

## Questions?

- Open a [Discussion](https://github.com/sunilkumarvalmiki/Open-Jarvis/discussions)
- Check existing issues and documentation
- Join our community chat (if available)

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project documentation

Thank you for contributing to Open-Jarvis! 🎉
