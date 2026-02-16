# Installation

## Requirements

- **Python 3.8+** — The SDK supports Python 3.8, 3.9, 3.10, 3.11, and 3.12
- **No runtime dependencies** — The core library is self-contained

!!! warning "Python 3.8 end-of-life"
    Python 3.8 reached end-of-life in October 2024 and no longer receives security updates. While the SDK still builds wheels for 3.8, we recommend **Python 3.10+** for production use.

## Installing with pip

```bash
pip install claim169
```

## Installing with uv

```bash
uv add claim169
```

## Installing with Poetry

```bash
poetry add claim169
```

## Development Installation

For contributing or testing with custom crypto providers:

```bash
# Clone the repository
git clone https://github.com/jeremi/claim-169.git
cd claim-169/core/claim169-python

# Install with development dependencies
uv sync --dev

# Build the native extension
maturin develop

# Run tests
uv run pytest tests/ -v
```

## Platform Support

Pre-built wheels are available for:

| Platform | Architecture | Wheel |
|----------|--------------|-------|
| Linux | x86_64 | `manylinux_2_28` |
| Linux | aarch64 | `manylinux_2_28` |
| macOS | x86_64 | `macosx_10_12` |
| macOS | arm64 (Apple Silicon) | `macosx_11_0` |
| Windows | x86_64 | `win_amd64` |

If a pre-built wheel is not available for your platform, pip will attempt to build from source, which requires:

- Rust 1.75+ toolchain
- maturin (`pip install maturin`)

## Verifying Installation

```python
import claim169

print(f"claim169 version: {claim169.version()}")
print(f"Python version: {claim169.__version__}")
```

Output:

```
claim169 version: 0.2.0-alpha
Python version: 0.2.0-alpha
```

## Optional Dependencies

For custom crypto providers (HSM, KMS integration), install the `cryptography` package:

```bash
pip install cryptography
```

This enables:

- Custom signature verification callbacks
- Custom decryption callbacks
- Integration with AWS KMS, Azure Key Vault, Google Cloud KMS

## Upgrading

```bash
pip install --upgrade claim169
```

## Uninstalling

```bash
pip uninstall claim169
```

## Troubleshooting Installation

### Build from Source Fails

If installing from source and the build fails:

1. Ensure Rust is installed: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Ensure maturin is installed: `pip install maturin`
3. Try building manually: `maturin build --release`

### Import Error

If you see `ImportError: No module named 'claim169'`:

1. Verify the installation: `pip show claim169`
2. Check your Python path: `python -c "import sys; print(sys.path)"`
3. Ensure you're using the correct Python interpreter

### Version Mismatch

If `claim169.version()` returns an unexpected version:

```bash
pip uninstall claim169
pip cache purge
pip install claim169
```
