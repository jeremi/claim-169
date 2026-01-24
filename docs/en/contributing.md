# Contributing

Thank you for your interest in contributing to Claim 169!

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Set up the development environment

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/claim-169.git
cd claim-169

# Build everything
cargo build --release
cargo test --all-features
```

## Development Setup

### Prerequisites

- **Rust** 1.75+ with cargo
- **Python** 3.8+ with maturin (for Python bindings)
- **Node.js** 18+ with npm (for TypeScript SDK)
- **wasm-pack** (for WebAssembly bindings)

### Building Components

```bash
# Rust core library
cargo build --release
cargo test --all-features

# Python bindings
cd core/claim169-python
maturin develop --release
uv run pytest tests/ -v

# TypeScript/WASM SDK
cd sdks/typescript
npm install
npm run build
npm test
```

## Commit Message Format

This project uses **Conventional Commits** for automatic changelog generation:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `perf` | Performance improvement |
| `refactor` | Code refactoring |
| `test` | Adding/updating tests |
| `chore` | Maintenance tasks |
| `ci` | CI/CD changes |

### Scopes

- `core` — Rust core library
- `python` — Python SDK
- `typescript` — TypeScript SDK
- `wasm` — WASM bindings
- `deps` — Dependencies

### Examples

```bash
feat(core): add support for palm biometrics
fix(python): handle empty QR code input
docs: update installation instructions
chore(deps): update ed25519-dalek to 2.1
```

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes with tests
3. Ensure all tests pass: `cargo test --all-features`
4. Run linting: `cargo clippy --all-targets --all-features`
5. Format code: `cargo fmt --all`
6. Submit a pull request

## Code Style

- Follow existing patterns in the codebase
- Write tests for new functionality
- Add documentation for public APIs
- Keep changes focused and atomic

## Reporting Issues

- Use the GitHub issue tracker
- Include reproduction steps
- Attach relevant logs or error messages
- Specify your environment (OS, SDK version)

## License

By contributing, you agree that your contributions will be licensed under the project's license.
