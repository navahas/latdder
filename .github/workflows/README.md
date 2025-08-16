# GitHub Actions Workflows

This directory contains automated workflows for the Latdder project.

## Workflows Overview

### 1. `ci.yml` - Continuous Integration
**Triggers**: Push to master, Pull requests  
**Purpose**: Comprehensive testing and validation

**Jobs**:
- **Test Suite**: Tests all feature combinations across Rust stable/beta/nightly
- **Linting**: Runs clippy on both library and binary with strict warnings
- **Documentation Check**: Validates documentation builds without warnings
- **Minimum Rust Version**: Ensures compatibility with Rust 1.75+

### 2. `formatter.yml` - Code Formatting
**Triggers**: Push, Pull requests  
**Purpose**: Automated code formatting with rustfmt

**Behavior**:
- **On Push**: Automatically formats code and commits changes
- **On PR**: Fails if formatting is needed (requires manual format)
- **Auto-commit**: Uses `[skip ci]` to prevent infinite loops

### 3. `docs.yml` - Documentation Publishing
**Triggers**: Push to master, Manual dispatch  
**Purpose**: Publishes documentation to GitHub Pages

**Features**:
- Builds complete documentation with all features
- Creates redirect from root to latdder docs
- Publishes to GitHub Pages automatically
- Accessible at: `https://username.github.io/latdder/`

### 4. `release.yml` - Release Automation
**Triggers**: Git tags matching `v*`  
**Purpose**: Automated release creation

**Process**:
- Builds and tests all features
- Generates comprehensive documentation
- Creates GitHub release with changelog
- Includes quick start instructions

## Setup Requirements

### GitHub Pages
1. Go to Repository Settings → Pages
2. Set Source to "GitHub Actions"
3. The docs workflow will automatically publish

### Permissions
The workflows require these permissions:
- `contents: read` - Read repository content
- `pages: write` - Publish to GitHub Pages  
- `id-token: write` - GitHub Pages deployment

### Secrets
No additional secrets required - uses built-in `GITHUB_TOKEN`.

## Usage

### Running Tests Locally
```bash
# Test specific feature
cargo test --features toast_level_01

# Run linting
cargo clippy --lib --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Generate docs
cargo doc --all-features --open
```

### Creating a Release
```bash
# Tag a release
git tag v0.1.0
git push origin v0.1.0

# Release workflow will automatically:
# 1. Build and test
# 2. Generate documentation  
# 3. Create GitHub release
```

### Manual Documentation Update
The docs workflow can be triggered manually:
1. Go to Actions tab
2. Select "Documentation" workflow
3. Click "Run workflow"

## Workflow Features

### Caching
- Cargo registry and index caching for faster builds
- Reduces build times on subsequent runs

### Matrix Testing
- Tests across multiple Rust versions (stable, beta, nightly)
- Tests all feature combinations individually
- Ensures broad compatibility

### Error Handling
- Strict clippy warnings (`-D warnings`)
- Documentation link checking
- Formatting enforcement

### Smart Formatting
- Auto-formats on push (commits changes)
- Fails on PR if formatting needed (requires manual action)
- Prevents formatting commit loops with `[skip ci]`

## Troubleshooting

### Documentation Fails to Publish
- Check GitHub Pages settings in repository
- Ensure `pages: write` permission is granted
- Verify workflow has completed successfully

### Tests Failing
- Check specific feature flags are correct
- Ensure all levels compile independently
- Verify no clippy warnings

### Formatting Issues
- Run `cargo fmt --all` locally before pushing
- Check that rustfmt.toml configuration is correct
- Ensure no merge conflicts in formatted code

## Customization

### Adding New Features
Update `ci.yml` matrix to include new feature flags:
```yaml
feature:
  - "init_level_01"
  - "toast_level_01"
  - "your_new_feature"  # Add here
```

### Changing Rust Version Requirements
Update `minimum-rust-version` job in `ci.yml`:
```yaml
- name: Setup Rust 1.75 (minimum supported)
  uses: dtolnay/rust-toolchain@master
  with:
    toolchain: "1.75"  # Change version here
```

### Documentation Customization
Modify `docs.yml` build step:
```yaml
- name: Build documentation
  run: |
    cargo doc --all-features --no-deps
    # Add custom documentation processing here
```