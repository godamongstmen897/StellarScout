# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- (Upcoming features go here)

### Changed
- (Changes go here)

### Fixed
- (Fixes go here)

### Deprecated
- (Deprecations go here)

### Removed
- (Removals go here)

### Security
- (Security fixes go here)

## [0.2.0] - 2026-05-18

### Added
- Code metrics analysis (function counts, code size, module statistics)
- Complexity detection (cyclomatic complexity, nesting depth analysis)
- Performance analysis (expensive operations detection, gas cost assessment)
- Human-readable and JSON output formats
- Modular analyzer architecture with reusable components
- CLI tool with format selection options
- GitHub Actions CI/CD pipeline
- Comprehensive contribution guidelines
- Security policy and bug bounty information

### Changed
- Refactored from placeholder to production-grade analyzer
- Updated dependencies (added serde, serde_json)
- Enhanced README with feature overview

### Security
- Initial security audit completed
- Dependency scanning enabled
- Secret scanning via TruffleHog

## [0.1.0] - 2026-05-17

### Added
- Project skeleton with analyzer and contract modules
- Basic WASM parsing capabilities
- Initial workspace setup

---

## Guidelines for Contributors

When adding changes, update this file with:
1. **Section**: Choose appropriate section (Added, Changed, Fixed, etc.)
2. **Description**: Clear, user-facing description
3. **References**: Link to issues/PRs like #123
4. **Format**: Keep consistent with established patterns

Example:
```markdown
### Added
- New feature for doing X (#456)
- Support for Y format in reports (#789)
```

### Versioning

- **MAJOR**: Breaking changes to public API
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes and minor improvements

Version bumps happen when:
- Release tag is created (vX.Y.Z)
- Published to crates.io
- GitHub release is created

See CONTRIBUTING.md for release process.
