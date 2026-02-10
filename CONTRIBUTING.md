# Contributing to Claim 169

Thank you for your interest in contributing to Claim 169! This guide will help you understand how to contribute effectively.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [AI-Assisted Development](#ai-assisted-development)
- [Security Considerations](#security-considerations)
- [Testing Requirements](#testing-requirements)
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)
- [Documentation](#documentation)
- [Questions?](#questions)
- [License](#license)

## Code of Conduct

This project is committed to providing a welcoming and inclusive environment for all contributors. Be respectful, constructive, and professional in all interactions.

## Getting Started

### Prerequisites

- **Rust** 1.75+ with cargo
- **Python** 3.8+ with uv and maturin (for Python SDK)
- **Node.js** 18+ with npm (for TypeScript SDK)
- **JDK** 17+ with Gradle (for Kotlin SDK)
- **wasm-pack** for WebAssembly builds (`cargo install wasm-pack`)

### Development Setup

```bash
# Clone the repository
git clone https://github.com/jeremi/claim-169.git
cd claim-169

# Build everything
cargo build --release

# Run tests
cargo test --all-features
```

### Building Individual Components

```bash
# Rust core library
cargo build -p claim169-core --release

# Python SDK
cd core/claim169-python
maturin develop --release

# TypeScript SDK
cd sdks/typescript
npm install
npm run build

# WASM bindings
cd core/claim169-wasm
wasm-pack build --target bundler --release

# Kotlin/Java SDK (build JNI native library first)
cargo build -p claim169-jni
cd sdks/kotlin && ./gradlew :claim169-core:test
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 2. Make Changes

- Write tests first (TDD approach preferred)
- Keep changes focused and minimal
- Follow existing code style and patterns

### 3. Test Your Changes

```bash
# Rust tests
cargo test --all-features

# Python tests
cd core/claim169-python && uv run pytest tests/ -v

# TypeScript tests
cd sdks/typescript && npm test

# Kotlin tests
cd sdks/kotlin && ./gradlew :claim169-core:test

# Lint checks
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

### Pre-commit (recommended)

This repo includes a `.pre-commit-config.yaml` with quick checks that match CI (formatting, docs checks).

```bash
uv sync --group dev
uv run pre-commit install

# Run manually
uv run pre-commit run --all-files
```

### 4. Commit Your Changes

We use [Conventional Commits](https://www.conventionalcommits.org/):

```bash
# Format: <type>(<scope>): <description>

# Examples:
git commit -m "feat(core): add support for palm biometrics"
git commit -m "fix(python): handle empty QR code input"
git commit -m "docs: update installation instructions"
```

#### Commit Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `perf` | Performance improvement |
| `refactor` | Code refactoring (no feature/fix) |
| `test` | Adding/updating tests |
| `chore` | Maintenance tasks |
| `ci` | CI/CD changes |
| `build` | Build system changes |

#### Scopes

- `core` - Rust core library
- `python` - Python SDK
- `typescript` - TypeScript SDK
- `kotlin` - Kotlin/Java SDK
- `wasm` - WASM bindings
- `deps` - Dependencies

### 5. Submit a Pull Request

1. Push your branch to your fork
2. Open a Pull Request against `main`
3. Fill out the PR template
4. Wait for CI checks to pass
5. Address any review feedback

## AI-Assisted Development

### Project Context

This project uses AI tools extensively in development. The maintainer uses multiple AI assistants (Claude, Codex, Cursor, etc.) as part of the standard development workflow. AI-assisted contributions are accepted, but the choice of tools is yours.

### Core Principle: Accountability

**You are responsible for your contributions.** Whether you use AI tools or write code manually, you remain fully accountable for:

- **Correctness** - The code does what it's supposed to do
- **Security** - No vulnerabilities or cryptographic weaknesses
- **Quality** - Code is maintainable, tested, and follows project standards
- **Licensing** - All code is compatible with the MIT license

### Disclosure Requirements

**External contributions must disclose AI use.** If you used AI tools (Codex, Claude, Copilot chat, etc.) to generate or substantially shape code in your PR, state so in the PR description. This helps reviewers calibrate their review — AI-generated code often looks plausible but can have subtle issues, especially in cryptographic or parsing logic.

**Format:**
```markdown
AI: Used [tool] to [what it helped with]. Validated by [how you verified].
```

**Examples:**
- `AI: Used Claude to draft the CBOR parsing logic. Validated against RFC 8949 examples and existing test vectors.`
- `AI: Used Copilot for boilerplate. Reviewed each suggestion manually.`

**Not required to disclose:**
- Line-level IDE autocomplete (Copilot inline suggestions, TabNine, etc.)
- Grammar/spelling assistance in documentation
- Using AI to understand the codebase or specifications

### Quality Standards (AI-assisted or not)

All contributions must:

1. **Build successfully** - `cargo build --all-features`
2. **Pass all tests** - `cargo test --all-features`
3. **Have no clippy warnings** - `cargo clippy --all-features`
4. **Be properly formatted** - `cargo fmt`
5. **Include appropriate tests** - Unit tests for logic, integration tests for workflows
6. **Handle errors correctly** - Use `Result` types, no unwrapping in library code

### Special Requirements for Cryptographic Code

For contributions involving cryptography, signatures, key handling, or security validations:

1. **Reference specifications** - Cite RFCs, standards, or academic papers
2. **Include security tests** - Test attack scenarios (weak keys, malformed inputs, edge cases)
3. **Explain security properties** - Document why the implementation is secure
4. **Be prepared for deeper review** - Crypto code requires expert review

**Warning:** AI tools often suggest cryptographic implementations that appear correct but have subtle security flaws (timing attacks, weak key handling, incorrect algorithm parameters). Always validate against specifications and include comprehensive security tests.

## Security Considerations

This project implements cryptographic protocols for digital identity. Security is paramount.

### Reporting Vulnerabilities

**Do not open public GitHub issues for security vulnerabilities.** Use [GitHub's private vulnerability reporting](https://github.com/jeremi/claim-169/security/advisories/new). See [SECURITY.md](SECURITY.md) for full details.

### Security-Critical Areas

- Key generation and handling
- Signature verification (Ed25519, ECDSA P-256)
- Encryption/decryption (AES-GCM)
- CBOR/COSE parsing and validation
- Decompression (zip bomb prevention)
- Timestamp validation

### Security Requirements

When contributing, please consider:

- Never commit secrets or credentials
- Avoid introducing new dependencies without review
- No constant-time violations in cryptographic operations
- Proper handling of weak/invalid keys
- Appropriate bounds checking and input validation
- Memory zeroization for sensitive data
- No panics in library code (use `Result` types)

## Testing Requirements

### Test Requirements

- **New features** - Include unit tests covering normal and error cases
- **Bug fixes** - Add a test that would have caught the bug
- **Cryptographic code** - Include test vectors from specifications
- **API changes** - Update integration tests
- Maintain or improve code coverage

### Test Categories

1. **Unit Tests**: Test individual functions/methods (in the same file as the code being tested)
2. **Integration Tests**: Test component interactions (in `tests/` directory)
3. **Conformance Tests**: Verify spec compliance using test vectors (in `test-vectors/` directory)
4. **Fuzz Tests**: Randomized input testing (in `fuzz/` directory)

### Running Specific Tests

```bash
# Single Rust test
cargo test test_name --all-features

# Run with verbose output
cargo test --all-features -- --nocapture

# Single Python test
cd core/claim169-python && uv run pytest tests/test_decode.py::test_name -v

# Single Kotlin test class
cd sdks/kotlin && ./gradlew :claim169-core:test --tests "fr.acn.claim169.DecodeValidTest"

# Generate test vectors
cargo run -p generate-vectors

# Fuzz tests (requires nightly)
cargo +nightly fuzz run fuzz_decode
```

## Code Style

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes without warnings
- Document public APIs with doc comments

### Python

- Follow PEP 8 style guide
- Use type hints for all public functions
- Run `ruff` for linting: `ruff check .`

### TypeScript

- Use TypeScript strict mode
- Provide JSDoc comments for public APIs
- Follow existing patterns in the codebase

### Kotlin/Java

- Follow [Kotlin coding conventions](https://kotlinlang.org/docs/coding-conventions.html)

## Pull Request Process

### Before Submitting

- [ ] Code builds without errors
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy --all-features`)
- [ ] Documentation is updated if needed
- [ ] CHANGELOG.md is updated for notable changes

### PR Description Template

```markdown
## Description

[Clear description of what this PR does]

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

[Describe how you tested your changes]

## Checklist

- [ ] All tests pass
- [ ] Code follows project style
- [ ] Documentation updated
- [ ] CHANGELOG.md updated

## Additional Context

[Any additional information reviewers should know]
```

### Review Process

- Maintainers will review PRs as time permits
- Be responsive to feedback and questions
- Be prepared to explain your implementation choices
- Expect iterative refinement — this is normal

### What May Be Rejected

PRs may be closed without merge if they:

- Don't build or pass tests
- Have security vulnerabilities
- Lack adequate testing
- Don't follow project standards
- Are out of scope for the project
- Show the contributor cannot explain or defend the changes

## Issue Reporting

### Bug Reports

Use the bug report template and include:

- Clear description of the bug
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, etc.)
- Minimal code example if possible

### Feature Requests

Use the feature request template and include:

- Clear description of the feature
- Use case and motivation
- Proposed API or implementation approach (if you have one)
- Willingness to implement it yourself

### Security Issues

**Do not open public issues for security vulnerabilities.** Use [GitHub's private vulnerability reporting](https://github.com/jeremi/claim-169/security/advisories/new).

## Documentation

- Update relevant README files for user-facing changes
- Add doc comments for new public APIs
- Update CHANGELOG.md for notable changes
- Keep examples working and up-to-date

### MkDocs site (multi-language)

This repo uses MkDocs Material + `mkdocs-static-i18n`:

- Source: `docs/<locale>/...` (currently `en`, `fr`)
- Config: `mkdocs.yml`

Preview docs locally:

```bash
uv sync --group dev
uv run mkdocs serve
```

Build the static site:

```bash
uv run mkdocs build
```

Translation workflow:

- Treat `docs/en/` as the source of truth.
- When adding a new page, prefer creating it for all locales to avoid mixed-language fallback.
- Keep page paths consistent across locales so the shared `nav` works.

## Additional Resources

- [MOSIP Claim 169 Specification](https://github.com/mosip/id-claim-169/tree/main)
- [Security Policy](SECURITY.md)
- [Changelog](CHANGELOG.md)

## Questions?

- Open a GitHub Discussion for general questions
- Check existing issues before creating new ones
- Ask in your PR if it's related to your contribution

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
