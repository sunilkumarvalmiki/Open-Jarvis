# Contributing to Open-Jarvis

Thank you for your interest in contributing to Open-Jarvis! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Style](#code-style)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Commit Messages](#commit-messages)

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and professional in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Open-Jarvis.git
   cd Open-Jarvis
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/sunilkumarvalmiki/Open-Jarvis.git
   ```
4. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Setting Up the Development Environment

1. **Install dependencies**:
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install Node.js dependencies (if package.json exists)
   npm install
   ```

2. **Install platform-specific dependencies**:
   - **Ubuntu/Debian**: `sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`
   - **macOS**: `xcode-select --install`
   - **Windows**: No additional dependencies required

3. **Run the application in development mode**:
   ```bash
   cd src-tauri
   cargo tauri dev
   ```

### Keeping Your Fork Updated

```bash
git fetch upstream
git checkout main
git merge upstream/main
git push origin main
```

## Code Style

### Rust

We follow the official Rust style guidelines and use `rustfmt` and `clippy` for code quality.

#### Formatting

Before submitting a PR, format your code:

```bash
cd src-tauri
cargo fmt
```

#### Linting

Ensure your code passes Clippy checks:

```bash
cd src-tauri
cargo clippy -- -D warnings
```

#### Rust Conventions

- Use snake_case for function and variable names
- Use CamelCase for types and structs
- Add documentation comments (`///`) for public APIs
- Keep functions focused and single-purpose
- Prefer explicit error handling over `.unwrap()`
- Use meaningful variable names

**Example:**

```rust
/// Organizes files in the specified directory by file type.
///
/// # Arguments
///
/// * `dir` - The directory path to organize
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error message on failure.
fn organize_files(dir: &Path) -> Result<(), String> {
    // Implementation
}
```

### JavaScript/HTML/CSS

- Use 2 spaces for indentation
- Use semicolons in JavaScript
- Use meaningful variable and function names
- Keep functions small and focused

## Testing

### Running Tests

```bash
cd src-tauri
cargo test
```

### Writing Tests

- Write unit tests for all new functions
- Place tests in the same file using `#[cfg(test)]` module
- Use descriptive test names that explain what is being tested

**Example:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organize_files_creates_directories() {
        // Test implementation
    }

    #[test]
    fn test_empty_recycle_bin_success() {
        // Test implementation
    }
}
```

### Test Coverage

- Aim for at least 70% code coverage for new features
- Critical paths should have 100% coverage
- Test both success and error cases

## Pull Request Process

1. **Ensure your code follows the style guidelines**:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```

2. **Update documentation** if you're changing functionality

3. **Create a pull request** with a clear title and description:
   - Reference any related issues (e.g., "Fixes #123")
   - Describe what changes you made and why
   - Include screenshots for UI changes
   - List any breaking changes

4. **Wait for review**:
   - Address any feedback from reviewers
   - Make requested changes in new commits
   - Once approved, your PR will be merged

### PR Template

When creating a PR, please include:

- **Description**: What does this PR do?
- **Related Issue**: Fixes #(issue number)
- **Type of Change**: Bug fix, new feature, documentation, etc.
- **Testing**: How was this tested?
- **Screenshots**: If applicable
- **Checklist**:
  - [ ] Code follows style guidelines
  - [ ] Tests added/updated
  - [ ] Documentation updated
  - [ ] No breaking changes (or documented)

## Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Code style changes (formatting, etc.)
- **refactor**: Code refactoring
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Maintenance tasks

### Examples

```
feat(mcp): add GitHub tool integration

Implements MCP client for GitHub operations including
repository management and issue tracking.

Fixes #42
```

```
fix(ui): correct button alignment on mobile

The action buttons were overlapping on small screens.
Updated CSS to use flexbox for better responsiveness.
```

```
docs(readme): update installation instructions

Added platform-specific dependency installation steps
for Ubuntu, macOS, and Windows.
```

## Issue Reporting

When reporting issues, please include:

- **Clear title** describing the problem
- **Steps to reproduce** the issue
- **Expected behavior** vs. actual behavior
- **Environment details**: OS, Rust version, Node version
- **Screenshots** if applicable
- **Logs** or error messages

## Feature Requests

For feature requests, please include:

- **Clear description** of the feature
- **Use case**: Why is this feature needed?
- **Proposed implementation**: If you have ideas
- **Alternatives considered**: Other approaches you've thought about

## Questions?

If you have questions about contributing, feel free to:

- Open an issue with the `question` label
- Start a discussion in GitHub Discussions
- Reach out to the maintainers

Thank you for contributing to Open-Jarvis! 🎉
