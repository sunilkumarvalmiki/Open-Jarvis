# Contributing to Open-Jarvis

Thank you for your interest in contributing to Open-Jarvis! This document provides guidelines and instructions for contributing.

## Code of Conduct

Be respectful and inclusive. We're building a welcoming community.

## Development Setup

### Prerequisites

- Rust 1.75 or higher
- Node.js 20 or higher
- pnpm 9 or higher
- For Linux: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`

### Setup Steps

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Open-Jarvis.git
   cd Open-Jarvis
   ```

3. Install dependencies:
   ```bash
   pnpm install
   ```

4. Create a branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Code Style

### Rust

- Follow Rust standard style guidelines
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings:
  ```bash
  cargo fmt --manifest-path src-tauri/Cargo.toml
  cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
  ```

### JavaScript/TypeScript

- Use 2 spaces for indentation
- Follow existing code style
- Run linter before committing (if configured)

## Commit Conventions

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```
feat(mcp): add GitHub MCP server integration

Implements GitHub MCP server for repository operations.
Includes list repos, create issue, and search code tools.

Closes #123
```

```
fix(ui): correct button alignment in settings panel

The save button was misaligned on smaller screens.
```

## Testing

### Rust Tests

```bash
cd src-tauri
cargo test
```

### Integration Tests

Ensure your changes don't break existing functionality:

```bash
pnpm tauri build
```

## Pull Request Process

1. **Update Documentation**: If you change functionality, update relevant docs
2. **Add Tests**: Include tests for new features or bug fixes
3. **Pass CI**: Ensure all CI checks pass
4. **Describe Changes**: Write a clear PR description explaining:
   - What changed
   - Why the change was needed
   - How to test the changes

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] `cargo fmt` and `cargo clippy` pass
- [ ] Tests pass locally
- [ ] Documentation updated (if needed)
- [ ] Commit messages follow conventions
- [ ] PR description is clear and complete

## Development Workflow

### Running in Development

```bash
pnpm tauri dev
```

### Building for Production

```bash
pnpm tauri build
```

### Debugging

- Use `console.log()` in frontend
- Use `println!()` or `dbg!()` in Rust
- Check Tauri console for errors

## Project Structure

```
Open-Jarvis/
├── src/                    # Frontend code
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── commands/       # Tauri commands
│   │   ├── ai/             # AI integration
│   │   ├── mcp/            # MCP client
│   │   └── config/         # Configuration
│   └── Cargo.toml
├── docs/                   # Documentation
└── tests/                  # Tests
```

## Areas for Contribution

- **MCP Integrations**: Add new MCP servers (GitHub, Jira, Slack, etc.)
- **AI Backends**: Support for additional LLM providers
- **UI/UX**: Improve the user interface
- **Documentation**: Improve docs and examples
- **Testing**: Add test coverage
- **Performance**: Optimize code performance

## Getting Help

- Open an issue for bugs or feature requests
- Start a discussion for questions or ideas
- Tag maintainers for PR reviews

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
