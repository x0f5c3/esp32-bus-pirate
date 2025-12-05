# GitHub Actions Workflows

This directory contains CI/CD pipelines for the ESP32 Bus Pirate project.

## Workflows

### [`ci.yml`](./workflows/ci.yml)
**Continuous Integration** - Runs on every push and pull request

**Jobs:**
- **check**: Format and lint Rust code
- **test**: Run unit and doc tests
- **build-docs**: Build mdBook documentation
- **conventional-commits**: Validate commit message format (PRs only)
- **build-rust**: Build firmware for ESP32-S3
- **status-check**: Aggregate status of all jobs

**Triggers:**
- Push to `main`, `develop`, or `copilot/**` branches
- Pull requests to `main` or `develop`

### [`deploy-docs.yml`](./workflows/deploy-docs.yml)
**Documentation Deployment** - Deploys mdBook to GitHub Pages

**Jobs:**
- **build**: Build documentation with mdBook and mermaid
- **deploy**: Deploy to GitHub Pages

**Triggers:**
- Push to `main` branch (when `book/**` changes)
- Manual workflow dispatch

**Requirements:**
- GitHub Pages must be enabled in repository settings
- Set to deploy from GitHub Actions

### [`release.yml`](./workflows/release.yml)
**Release Automation** - Creates releases when tags are pushed

**Jobs:**
- **create-release**: Generate changelog with git-cliff and create GitHub release
- **build-firmware**: Build firmware binaries for all targets
- **build-docs-release**: Build and attach documentation
- **publish-crates**: Publish crates to crates.io (optional)

**Triggers:**
- Tags matching `v*` (e.g., `v1.0.0`)
- Branches matching `release/v*` (for release preparation)

**Creating a Release:**
```bash
# 1. Update version in Cargo.toml files
# 2. Commit changes
git add .
git commit -m "chore(release): prepare for v1.0.0"

# 3. Create and push tag
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

## Secrets Required

Configure these secrets in repository settings:

- **`GITHUB_TOKEN`**: Automatically provided by GitHub Actions
- **`CARGO_REGISTRY_TOKEN`**: For publishing to crates.io (optional)
  - Get from https://crates.io/settings/tokens
  - Add to repository secrets

## Caching Strategy

All workflows use GitHub Actions cache to speed up builds:

- **Cargo registry**: `~/.cargo/registry`
- **Cargo git**: `~/.cargo/git`
- **Build artifacts**: `rust/target`

Cache keys include `Cargo.lock` hash for automatic invalidation.

## Status Badges

Add to README.md:

```markdown
![CI](https://github.com/x0f5c3/esp32-bus-pirate/workflows/CI%20-%20Build%20and%20Test/badge.svg)
![Docs](https://github.com/x0f5c3/esp32-bus-pirate/workflows/Deploy%20mdBook%20to%20GitHub%20Pages/badge.svg)
```

## Monitoring

- **Actions tab**: View all workflow runs
- **Pull requests**: See status checks before merging
- **Releases**: Automatically created with changelog and artifacts

## Troubleshooting

### Workflow failing on `cargo build`

Check:
1. All `Cargo.toml` files have correct dependencies
2. `rust-toolchain.toml` specifies correct Rust version
3. espup installation succeeds

### Documentation deployment failing

Check:
1. GitHub Pages is enabled
2. Source is set to "GitHub Actions"
3. `book/book.toml` is valid
4. All referenced files exist

### Release workflow not triggering

Check:
1. Tag format matches `v*` (e.g., `v1.0.0`, not `1.0.0`)
2. Tag is pushed to remote: `git push origin v1.0.0`
3. Workflow file exists on the branch where tag is created

## Local Testing

Test workflows locally with [act](https://github.com/nektos/act):

```bash
# Install act
brew install act  # macOS
# or
sudo apt install act  # Linux

# Run CI workflow
act push

# Run specific job
act -j check

# Run with secrets
act -s CARGO_REGISTRY_TOKEN=xxx
```

## Customization

### Adding a New Job

1. Edit the appropriate workflow file
2. Add job under `jobs:` section
3. Test locally if possible
4. Commit and push to test on GitHub

### Modifying Build Matrix

Edit `strategy.matrix` in `ci.yml` or `release.yml`:

```yaml
strategy:
  matrix:
    target:
      - xtensa-esp32s3-none-elf
      - xtensa-esp32-none-elf  # Add ESP32 classic
```

## Performance

- **Average CI time**: ~5-10 minutes
- **Cache hit**: ~2-3 minutes
- **Documentation build**: ~1 minute
- **Release build**: ~10-15 minutes

## See Also

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust GitHub Actions](https://github.com/actions-rs)
- [git-cliff Configuration](../../cliff.toml)
