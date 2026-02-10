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
- **Python** 3.8+ with uv and maturin (for Python bindings)
- **Node.js** 18+ with npm (for TypeScript SDK)
- **JDK** 17+ with Gradle (for Kotlin SDK)
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

# Kotlin/Java SDK
cargo build -p claim169-jni
cd sdks/kotlin && ./gradlew :claim169-core:test
```

## AI-Assisted Development

This project uses AI tools extensively in development. AI-assisted contributions are accepted, but the choice of tools is yours.

### Accountability

**You are responsible for your contributions.** Whether you use AI tools or write code manually, you remain fully accountable for correctness, security, quality, and licensing compatibility.

### Disclosure

**External contributions must disclose AI use.** If you used AI tools (Codex, Claude, Copilot chat, etc.) to generate or substantially shape code in your PR, state so in the PR description. This helps reviewers calibrate their review — AI-generated code often looks plausible but can have subtle issues, especially in cryptographic or parsing logic.

Format:

```
AI: Used [tool] to [what it helped with]. Validated by [how you verified].
```

Line-level IDE autocomplete, documentation grammar help, and using AI to understand the codebase do not require disclosure.

!!! warning "Cryptographic code"
    AI tools often suggest cryptographic implementations that appear correct but have subtle security flaws (timing attacks, weak key handling, incorrect algorithm parameters). Always validate against specifications and include comprehensive security tests.

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
- `kotlin` — Kotlin/Java SDK
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
6. Submit a pull request using the provided template

The PR template includes an **AI Disclosure** section — fill it in if you used AI tools, or delete it if you didn't.

## Code Style

- Follow existing patterns in the codebase
- Write tests for new functionality
- Add documentation for public APIs
- Keep changes focused and atomic

## Security

**Do not open public GitHub issues for security vulnerabilities.** Use [GitHub's private vulnerability reporting](https://github.com/jeremi/claim-169/security/advisories/new). See [SECURITY.md](https://github.com/jeremi/claim-169/blob/main/SECURITY.md) for full details.

## Reporting Issues

- Use the GitHub issue tracker
- Include reproduction steps
- Attach relevant logs or error messages
- Specify your environment (OS, SDK version)

## License

By contributing, you agree that your contributions will be licensed under the project's license.
