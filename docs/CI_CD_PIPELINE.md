# CI/CD Pipeline Documentation

## Overview

StellarScout has a comprehensive CI/CD pipeline protecting code quality, security, and reliability before any code reaches the main branch.

## Pipeline Stages

### 1. Pull Request Stage
Every PR automatically runs:

```
✓ Unit Tests (stable + nightly)
✓ Code Formatting (rustfmt)
✓ Linting (clippy)
✓ Code Coverage
✓ Security Audit
✓ Dependency Scanning
✓ Secret Scanning
✓ AI Code Review
```

### 2. Review Stage
- Human review (minimum 1 approval)
- Copilot AI code analysis
- Automated checks pass before review possible

### 3. Merge Stage
- All checks must pass
- Merge to main only when approved
- Automatic deployment of docs

### 4. Release Stage (on tag)
- Multi-platform builds (Linux, macOS, Windows)
- Binary artifacts created
- Automatic GitHub release
- Publication to crates.io

## GitHub Actions Workflows

| Workflow | Triggers | Purpose |
|----------|----------|---------|
| **tests.yml** | Push/PR | Tests, formatting, coverage |
| **security.yml** | Push/PR/Weekly | Audits, dependency checks |
| **release.yml** | Tags/Manual | Builds & publication |
| **docs.yml** | Push/PR | Documentation generation |
| **code-review.yml** | PR | AI code review |

## Configuration Files

- `.github/workflows/` - GitHub Actions definitions
- `deny.toml` - Dependency security rules
- `clippy.toml` - Linting configuration
- `.githooks/pre-commit` - Local validation hook

## Local Development

### Setup Pre-commit Hook

```bash
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit
```

### Before Committing

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

## Protection Rules

**Main branch requires**:
- ✓ All status checks pass
- ✓ Code review approved
- ✓ No outdated branches
- ✓ Admin enforcement enabled

## Security Features

✅ Dependency auditing
✅ Secret scanning
✅ License compliance checking
✅ Multi-platform testing
✅ Code coverage tracking

## Release Process

1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create git tag: `git tag -a vX.Y.Z`
4. Push tag: `git push origin vX.Y.Z`
5. GitHub Actions automatically:
   - Builds for all platforms
   - Creates GitHub release
   - Publishes to crates.io

See CONTRIBUTING.md and SECURITY.md for more details.
