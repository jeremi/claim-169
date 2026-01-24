# Contributing to claim-169

Thank you for your interest in contributing to the MOSIP Claim 169 library! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and constructive in all interactions. We welcome contributors of all experience levels.

## Getting Started

### Prerequisites

- **Rust** 1.70+ with cargo
- **Python** 3.8+ with uv (for Python SDK)
- **Node.js** 18+ with npm (for TypeScript SDK)
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
- `wasm` - WASM bindings
- `deps` - Dependencies

### 5. Submit a Pull Request

1. Push your branch to your fork
2. Open a Pull Request against `main`
3. Fill out the PR template
4. Wait for CI checks to pass
5. Address any review feedback

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

## Testing Guidelines

### Test Requirements

- All new features must have tests
- Bug fixes should include regression tests
- Maintain or improve code coverage

### Test Categories

1. **Unit Tests**: Test individual functions/methods
2. **Integration Tests**: Test component interactions
3. **Conformance Tests**: Verify spec compliance using test vectors

### Running Specific Tests

```bash
# Single Rust test
cargo test test_name --all-features

# Single Python test
cd core/claim169-python && uv run pytest tests/test_decode.py::test_name -v

# Generate test vectors
cargo run -p generate-vectors
```

## Security

### Reporting Vulnerabilities

Please report security vulnerabilities to **security@openspp.org**. Do not use public GitHub issues for security reports.

### Security Considerations

When contributing, please consider:

- Never commit secrets or credentials
- Avoid introducing new dependencies without review
- Follow secure coding practices
- Be careful with cryptographic operations

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

## Questions?

- Open a GitHub Discussion for general questions
- Check existing issues before creating new ones
- Join our community channels (if available)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
