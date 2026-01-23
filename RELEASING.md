# Releasing

This document describes how to release new versions of the Claim 169 libraries to crates.io, PyPI, and npm.

## Prerequisites

Before your first release, you need to:

1. Have accounts on all three registries
2. Configure GitHub repository secrets and environments
3. Verify package names are available

### Registry Accounts

| Registry | Create Account | Package Name |
|----------|---------------|--------------|
| [crates.io](https://crates.io) | Sign in with GitHub | `claim169-core` |
| [PyPI](https://pypi.org) | Create account | `claim169` |
| [npm](https://npmjs.com) | Create account | `claim169` |

## First-Time Setup

### 1. crates.io (Rust)

Create an API token:

1. Go to https://crates.io/settings/tokens
2. Click "New Token"
3. Name: `github-actions-claim169`
4. Scopes: `publish-new`, `publish-update`
5. Copy the token

Add to GitHub:

1. Go to repository **Settings → Secrets and variables → Actions**
2. Click "New repository secret"
3. Name: `CRATES_IO_TOKEN`
4. Value: paste the token

Verify the crate name is available:

```bash
cargo search claim169-core
# Should return no results or your package
```

### 2. PyPI (Python)

PyPI uses **OIDC Trusted Publishing** which is more secure than API tokens. No secret is needed, but you must configure PyPI to trust your GitHub workflow.

Configure trusted publisher on PyPI:

1. Go to https://pypi.org/manage/account/publishing/
2. Under "Add a new pending publisher", fill in:
   - **PyPI Project Name**: `claim169`
   - **Owner**: `jeremi` (your GitHub username)
   - **Repository name**: `claim-169`
   - **Workflow name**: `publish-release.yml`
   - **Environment name**: `pypi`
3. Click "Add"

Create GitHub environment:

1. Go to repository **Settings → Environments**
2. Click "New environment"
3. Name: `pypi`
4. (Optional) Add deployment protection rules:
   - Required reviewers
   - Wait timer

### 3. npm (TypeScript/WASM)

Create an access token:

1. Go to https://www.npmjs.com/settings (your profile → Access Tokens)
2. Click "Generate New Token" → "Granular Access Token"
3. Configure:
   - **Token name**: `github-actions-claim169`
   - **Expiration**: No expiration (or set a reminder to rotate)
   - **Packages and scopes**: Read and write
   - **Select packages**: Only select packages → `claim169`
4. Generate and copy the token

Add to GitHub:

1. Go to repository **Settings → Secrets and variables → Actions**
2. Click "New repository secret"
3. Name: `NPM_TOKEN`
4. Value: paste the token

Create GitHub environment:

1. Go to repository **Settings → Environments**
2. Click "New environment"
3. Name: `npm`
4. (Optional) Add deployment protection rules

Verify package name is available:

```bash
npm view claim169
# Should return 404 or your package info
```

## Release Process

### Step 1: Prepare the Release

Run the "Prepare Release" workflow:

1. Go to **Actions → Prepare Release**
2. Click "Run workflow"
3. Enter version (e.g., `0.1.0`, `0.2.0-alpha`)
4. Click "Run workflow"

This will:
- Validate the version format
- Bump versions in all package files
- Generate/update CHANGELOG.md
- Create a release PR

### Step 2: Review the Release PR

The PR will contain:
- Version bumps in `Cargo.toml`, `pyproject.toml`, `package.json`
- Updated `CHANGELOG.md`

Review checklist:
- [ ] Version numbers are correct in all files
- [ ] Changelog accurately reflects changes
- [ ] CI passes

Merge the PR when ready.

### Step 3: Create and Push the Tag

After merging, create a GPG-signed tag:

```bash
git checkout main
git pull origin main
git tag -s v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

If you don't have GPG set up, use an annotated tag:

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### Step 4: Monitor the Release

Pushing the tag triggers the publish workflow:

1. Go to **Actions → Publish Release**
2. Monitor the jobs:
   - CI (runs all tests)
   - Build Python Wheels (all platforms)
   - Build npm Package
   - Publish to crates.io
   - Publish to PyPI
   - Publish to npm
   - Create GitHub Release

### Step 5: Verify the Release

Check each registry:

```bash
# crates.io
cargo search claim169-core

# PyPI
pip index versions claim169

# npm
npm view claim169
```

## Pre-release Versions

Use pre-release suffixes for alpha/beta releases:

| Version | Type | npm tag |
|---------|------|---------|
| `0.2.0-alpha` | Alpha | `alpha` |
| `0.2.0-beta` | Beta | `beta` |
| `0.2.0-rc.1` | Release Candidate | `rc` |
| `0.2.0` | Stable | `latest` |

Installing pre-releases:

```bash
# npm
npm install claim169@alpha
npm install claim169@beta

# pip
pip install claim169 --pre

# cargo
cargo add claim169-core@0.2.0-alpha
```

## Version Bumping

The release workflow automatically bumps versions in:

| File | Field |
|------|-------|
| `Cargo.toml` | workspace `version` |
| `core/claim169-python/pyproject.toml` | `version` |
| `sdks/typescript/package.json` | `version` |
| `README.md` | dependency examples |
| `core/claim169-core/README.md` | dependency examples |
| `docs/{en,es,fr}/getting-started/installation.md` | dependency examples |
| `docs/{en,es,fr}/guides/versioning.md` | dependency examples |

## Troubleshooting

### "crate already exists" error

The crate name is already taken on crates.io. You'll need to choose a different name.

### PyPI trusted publishing fails

Common causes:
- Environment name mismatch (must be exactly `pypi`)
- Workflow filename mismatch (must be `publish-release.yml`)
- Repository name mismatch
- Publisher not configured on PyPI

Verify your configuration at https://pypi.org/manage/account/publishing/

### npm provenance fails

Ensure:
- The `npm` environment exists in GitHub
- `id-token: write` permission is set in workflow
- `NPM_TOKEN` secret has correct permissions

### Tag already exists

If you need to re-release the same version (e.g., after fixing a failed publish):

```bash
# Delete local tag
git tag -d v0.1.0

# Delete remote tag
git push origin :refs/tags/v0.1.0

# Re-create and push
git tag -s v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

**Warning**: Only do this if the packages weren't published. You cannot overwrite published packages on crates.io or npm.

### Yanking a bad release

If you publish a broken version:

```bash
# crates.io - yank (doesn't delete, prevents new installs)
cargo yank claim169-core@0.1.0

# PyPI - yank
pip install twine
twine yank claim169 0.1.0

# npm - deprecate (cannot fully remove)
npm deprecate claim169@0.1.0 "This version has critical bugs, please upgrade"
```

## Dry Run

To test the release process without publishing:

1. Run "Prepare Release" workflow with `dry_run: true`
2. Review the output to see what would change
3. No PR is created, no versions are bumped

## Security

- Never commit API tokens to the repository
- Use GitHub Environments with protection rules for production releases
- Prefer OIDC trusted publishing (PyPI) over long-lived tokens
- Rotate npm token periodically
- Use GPG-signed tags for releases
