# Contributing to Open-Jarvis

Thank you for your interest in contributing to Open-Jarvis! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive feedback
- Respect different viewpoints and experiences

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/sunilkumarvalmiki/Open-Jarvis/issues)
2. If not, create a new issue using the bug report template
3. Provide as much detail as possible:
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - System information (OS, Rust version, etc.)
   - Screenshots if applicable

### Suggesting Features

1. Check if the feature has already been requested
2. Create a new issue using the feature request template
3. Clearly describe:
   - The problem the feature would solve
   - How you envision the feature working
   - Any alternative solutions you've considered

### Pull Requests

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/Open-Jarvis.git
   cd Open-Jarvis
   ```

2. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

3. **Make Your Changes**
   - Follow the code style guidelines below
   - Write or update tests as needed
   - Update documentation if necessary

4. **Test Your Changes**
   ```bash
   # Format code
   cargo fmt --manifest-path src-tauri/Cargo.toml
   
   # Run linter
   cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
   
   # Run tests
   cargo test --manifest-path src-tauri/Cargo.toml
   ```

5. **Commit Your Changes**
   ```bash
   git add .
   git commit -m "feat: add new feature" # or "fix: resolve bug"
   ```
   
   Use conventional commit messages:
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation changes
   - `style:` - Code style changes (formatting, etc.)
   - `refactor:` - Code refactoring
   - `test:` - Adding or updating tests
   - `chore:` - Maintenance tasks

6. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   
   Then create a Pull Request on GitHub using the PR template.

## Development Setup

### Prerequisites

- Rust 1.75 or higher
- Node.js 20 or higher
- Platform-specific dependencies (see README.md)

### Building

```bash
# Development build with hot-reload
cargo tauri dev --manifest-path src-tauri/Cargo.toml

# Production build
cargo tauri build --manifest-path src-tauri/Cargo.toml
```

### Testing

```bash
# Run all tests
cargo test --manifest-path src-tauri/Cargo.toml

# Run specific test
cargo test --manifest-path src-tauri/Cargo.toml test_name

# Run tests with output
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture
```

## Code Style Guidelines

### Rust Code

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Use `cargo clippy` to catch common mistakes
- Write idiomatic Rust code
- Add documentation comments (`///`) for public APIs
- Keep functions focused and small
- Use meaningful variable and function names

#### Rust Examples

```rust
/// Opens a URL in the default browser with a delay.
///
/// # Arguments
///
/// * `app` - The Tauri app handle
/// * `url` - The URL to open
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error string on failure
#[tauri::command]
fn open_browser(app: tauri::AppHandle, url: String) -> Result<(), String> {
    // Implementation
}
```

### Frontend Code

- Use consistent indentation (2 spaces)
- Use meaningful CSS class names
- Keep JavaScript simple and readable
- Avoid inline styles when possible
- Comment complex logic

### File Organization

- Keep related code together
- Use modules to organize functionality
- Place tests near the code they test
- Keep configuration files in the root directory

## Git Workflow

1. **Main Branch**: `main` - Production-ready code
2. **Development Branch**: `develop` - Integration branch for features
3. **Feature Branches**: `feature/feature-name` - New features
4. **Bug Fix Branches**: `fix/bug-name` - Bug fixes
5. **Release Branches**: `release/version` - Release preparation

### Branch Naming

- `feature/add-voice-commands` - New features
- `fix/crash-on-startup` - Bug fixes
- `docs/update-readme` - Documentation
- `refactor/cleanup-mcp-module` - Refactoring
- `test/add-unit-tests` - Tests

## Testing Requirements

- Add tests for new features
- Ensure all tests pass before submitting PR
- Maintain or improve code coverage
- Test on your platform (Windows, macOS, or Linux)
- Include integration tests for new commands

## Documentation

- Update README.md if you change user-facing features
- Update architecture docs for structural changes
- Add inline comments for complex logic
- Write clear commit messages
- Update CHANGELOG.md for notable changes

## Security

- Never commit secrets, API keys, or credentials
- Report security vulnerabilities privately (see SECURITY.md)
- Follow secure coding practices
- Validate all user inputs
- Use Rust's type system for safety

## Review Process

1. **Automated Checks**: CI must pass
2. **Code Review**: At least one maintainer approval required
3. **Testing**: Changes must be tested
4. **Documentation**: Documentation must be updated if needed

### What Reviewers Look For

- Code quality and style
- Test coverage
- Documentation
- Performance implications
- Security concerns
- Backward compatibility

## Getting Help

- 💬 [GitHub Discussions](https://github.com/sunilkumarvalmiki/Open-Jarvis/discussions) - Ask questions
- 🐛 [GitHub Issues](https://github.com/sunilkumarvalmiki/Open-Jarvis/issues) - Report bugs
- 📖 [Documentation](docs/) - Read the docs

## License

By contributing to Open-Jarvis, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in:
- The project's README.md
- Release notes for significant contributions
- GitHub's contributor graph

Thank you for contributing to Open-Jarvis! 🎉
