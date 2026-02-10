# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-alpha] - 2026-02-10

### Bug Fixes

- **core,typescript**: Harden X.509 header parsing and validation
- **kotlin**: Harden JNI bindings and Kotlin SDK security
- **docs**: Restructure Kotlin API reference with proper heading hierarchy
- Normalize trailing whitespace in spec and test vectors
- **ci**: Use virtualenv for maturin develop in docs workflow
- **docs**: Revert Python API docs to static content
- **ci**: Resolve codespell and conformance test failures
- **ci**: Fix codespell ignore list and conformance Gson serialization
- **kotlin**: Align Claim169NotFound error message with core error type
- **playground**: Fix camera leak, QR export scannability, and trim demo data

### Build

- **docs**: Add API doc generation pipeline with drift checking

### CI/CD

- Add Maven Central publishing and harden release workflows
- **kotlin**: Allow passwordless GPG_PRIVATE_KEY signing
- Add custom CodeQL workflow for java-kotlin analysis
- **codeql**: Upgrade CodeQL action from v3 to v4
- **docs**: Build wasm before generating ts api docs

### Dependencies

- **deps**: Update Python SDK lockfile

### Documentation

- Document GPG signing key for releases
- Document PEM and Base64 key format support
- Update claim 169 specification to v1.2
- Add Kotlin/Java SDK documentation
- Improve onboarding (test vectors, Base45 warning)
- **fr**: Add kotlin quick summaries
- **python**: Add comprehensive docstrings and text_signature attributes
- **fr**: Polish photo compression guide
- Include photo guide in nav and gate deploy after docs
- Add AI disclosure policy, PR template, and contributor guide improvements

### Features

- **core**: Add PEM public key format support
- **playground**: Add enhanced error display and key format detection
- **core**: Add COSE X.509 header extraction (x5bag, x5chain, x5t, x5u)
- **python**: Expose X509Headers in Python SDK
- **typescript**: Add X509Headers interface and WASM bindings
- **playground**: Add locationCode, legalStatus, countryOfIssuance fields
- **kotlin**: Add Kotlin/JVM SDK with UniFFI bindings
- **kotlin**: Improve SDK ergonomics and callback handling
- **kotlin**: Use enum constants in tests and add interop test suite
- **kotlin**: Add Java interop annotations and separate Java docs
- **playground**: Add photo upload with canvas-based compression
- **playground**: Add photo lightbox, blur optimization, and compression guide

### Miscellaneous

- Add .gradle to gitignore and remove cached build files

### Refactoring

- **kotlin**: Rename package from org.mosip.claim169 to org.acn.claim169
- **kotlin**: Extract mutex helpers and remove duplicate test vectors
- **kotlin**: Rename package org.acn to fr.acn [**BREAKING**]
- **kotlin**: Expose SDK models/errors under fr.acn.claim169 [**BREAKING**]

### Testing

- Add bestQualityFingers to demographics-full test vector
- **kotlin**: Expand test coverage from 38 to 66 tests

## [0.1.0-alpha.3] - 2026-01-24

### Bug Fixes

- **ci**: Satisfy clippy warnings
- **docs**: Avoid innerHTML in mermaid init
- **release**: Remove non-existent doc paths from bump-version script

### Documentation

- Consolidate docs and clarify security model

### Features

- Add custom crypto provider support for HSM and cloud KMS integration
- Add FR docs, mermaid homepage, and crypto API tweaks

### Miscellaneous

- Add pre-commit hooks
- Normalize whitespace

### Performance

- **ci**: Use pre-built binaries for git-cliff and wasm-pack
## [0.1.0-alpha.2] - 2026-01-23

### Bug Fixes

- **release**: Update pip/npm version patterns in versioning.md

### Documentation

- Improve READMEs and add version auto-update to release workflow
## [0.1.0-alpha] - 2026-01-23

### Bug Fixes

- Address code review feedback for spec compliance
- Resolve clippy lints in test files
- Resolve clippy lints in Python and WASM bindings
- Resolve clippy warnings in generate-vectors
- **ci**: Add 'all' subcommand to generate-vectors
- **ci**: Python virtualenv and WASM bigint type
- **ci**: Correct Python and TypeScript test setup
- **test**: Handle ed25519 weak key rejection in Python test
- **ci**: Use wheel install for Conformance job Python SDK
- **ci**: Use maturin develop instead of wheel install
- **ci**: Disable timestamp validation in conformance tests
- **ci**: Correct rust-toolchain action name
- **ci**: Use valid vitest reporter (dot instead of basic)
- **playground**: Update docs link to point to deployed docs
- **ci**: Use ESM import for npm package test
- **ci**: Include wasm files in npm package
- **ci**: Verify npm package structure instead of runtime import
- **docs**: Add hook to copy sitemap.xml to all page directories
- **playground**: Prevent double camera initialization in React Strict Mode
- **playground**: Show skipped status for COSE_Sign1 when unverified
- **ci**: Remove non-existent release label from PR creation
- **ci**: Add workflow_call trigger to enable reusable workflow
- **ci**: Use auto manylinux for Python wheel builds
- **ci**: Set up Python before maturin-action for wheel builds
- **ci**: Simplify Python wheel builds, remove aarch64-linux cross-compile
- **ci**: Drop macOS Intel, keep Linux x64, macOS arm64, Windows x64

### CI

- Install maturin via uv dev deps
- Install a single Python wheel in conformance
- Avoid pip conflicts when multiple wheels exist

### CI/CD

- Add release automation with multi-registry publishing
- Fix conformance tests and optimize wasm-pack caching
- Add code coverage with Codecov integration
- Add workflow_dispatch trigger to coverage workflow
- Merge coverage into CI workflow as conditional job
- Add test-packaging job to validate packages before release
- **docs**: Check internal links and anchors
- **docs**: Fix codespell config and avoid FR/ES false positives
- Add explicit permissions to workflows
- Optimize caching and parallelize jobs

### Conformance

- Allow unverified decode

### Dependencies

- **deps**: Upgrade dependencies to latest versions

### Docs

- Align with verified-by-default decode APIs

### Documentation

- Add comprehensive documentation and examples
- Add MkDocs documentation site with i18n support
- Update Rust examples to use builder pattern
- Improve multilingual documentation
- Add i18n nav translations and docs quality gates
- Add contributing guide and link from home
- **en**: Simplify homepage layout
- Add RELEASING.md with first-time setup and release process
- Add playground screenshot to documentation
- Move screenshot to top of playground pages
- Consolidate unreleased changes into 0.1.0-alpha
- Add GitHub workflow watching tip to CLAUDE.md

### Features

- Initial implementation of Claim 169 QR code library
- **core**: Add encoder support and fix codex review issues
- **typescript**: Add signature verification and decryption to Decoder [**BREAKING**]
- **python**: Rename decode() to decode_unverified() for explicit API [**BREAKING**]
- Add interactive web playground
- **playground**: Add sample data and decode examples
- **core**: Add builder methods to Claim169
- **python,typescript**: Add decode() convenience functions
- **playground**: Add local language name support with Thai sample data
- **playground**: Redesign UI with jwt.io-style layout
- **playground**: Improve UX with key generation and credential settings grouping

### Miscellaneous

- Add MIT LICENSE file
- Update package-lock.json with coverage dependency

### Rustfmt

- Fix blank lines

### Styling

- Format Rust code

### Testing

- **core**: Improve test coverage for low-coverage modules
- **python,typescript**: Lock documented convenience APIs
---
*Generated by [git-cliff](https://git-cliff.org)*
