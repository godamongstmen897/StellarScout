# Contributing to StellarScout

Thank you for your interest in contributing to StellarScout! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project is committed to providing a welcoming and inclusive environment for all contributors. Please read our [Code of Conduct](CODE_OF_CONDUCT.md).

## Getting Started

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Cargo (comes with Rust)
- Git

### Setup Development Environment

```bash
# Clone the repository
git clone https://github.com/godamongstmen897/StellarScout.git
cd StellarScout

# Install pre-commit hooks
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit

# Build the project
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/short-description
# or for bugs:
git checkout -b fix/short-description
```

### 2. Make Your Changes

- Write clear, focused commits
- Follow Rust conventions and project style
- Add tests for new functionality
- Update documentation as needed

### 3. Run Quality Checks

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all

# Generate documentation
cargo doc --no-deps --open
```

### 4. Commit Your Work

```bash
git commit -m "type: brief description

Longer description explaining the changes made.

Fixes #123
```

**Commit types:**
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Test additions/improvements
- `refactor:` - Code refactoring
- `perf:` - Performance improvements
- `chore:` - Build, CI, dependencies

### 5. Push and Create a Pull Request

```bash
git push origin feature/short-description
```

Then create a PR on GitHub. Use the PR template provided.

## Code Style

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Address all `clippy` warnings
- Write clear comments for complex logic
- Keep lines under 100 characters when reasonable

## Testing

- Add tests for new features
- Ensure all tests pass: `cargo test --all`
- Aim for >80% code coverage
- Test edge cases and error scenarios

## Documentation

- Update README.md for user-facing changes
- Add doc comments to public APIs
- Update CHANGELOG.md
- Include examples in documentation

## Security

- Do not commit secrets or credentials
- Report security vulnerabilities privately (see SECURITY.md)
- Follow Rust security best practices
- Scan dependencies: `cargo deny check`

## CI/CD Pipeline

The project uses GitHub Actions for:
- Running tests on multiple Rust versions
- Code coverage tracking
- Security audits
- Linting and formatting checks
- Building release binaries

Your PR must pass all checks before merging.

## Release Process

Releases follow semantic versioning (MAJOR.MINOR.PATCH):

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create a git tag: `git tag -a vX.Y.Z -m "Release X.Y.Z"`
4. Push tag: `git push origin vX.Y.Z`
5. GitHub Actions automatically builds and publishes

## Getting Help

- **Documentation**: See `/docs` directory
- **Issues**: Search existing issues or create a new one
- **Discussions**: Start a discussion for questions
- **Community**: Join our Stellar community channels

## Review Process

Pull requests are reviewed by maintainers for:
- Code quality and style
- Security concerns
- Test coverage
- Documentation completeness
- Alignment with project goals

Thank you for contributing! 🌟
