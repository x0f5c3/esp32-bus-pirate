# Conventional Commits

## Overview

This project uses the [Conventional Commits](https://www.conventionalcommits.org/) specification for all commit messages. This provides a structured format that enables:

- **Automated Changelog Generation**: Tools can parse commits to generate release notes
- **Semantic Versioning**: Automatically determine version bumps (major/minor/patch)
- **Better Git History**: Clear, searchable commit history
- **CI/CD Integration**: Trigger different actions based on commit type

## Format

```
<type>(<scope>): <subject>

[optional body]

[optional footer(s)]
```

### Type

The **type** must be one of the following:

| Type | Description | Version Bump |
|------|-------------|--------------|
| `feat` | A new feature | MINOR |
| `fix` | A bug fix | PATCH |
| `docs` | Documentation only changes | N/A |
| `style` | Code style changes (formatting, missing semi-colons, etc.) | N/A |
| `refactor` | Code refactoring (neither fixes a bug nor adds a feature) | N/A |
| `perf` | Performance improvements | PATCH |
| `test` | Adding or modifying tests | N/A |
| `build` | Changes to build system or dependencies | N/A |
| `ci` | Changes to CI/CD configuration | N/A |
| `chore` | Other changes that don't modify src or test files | N/A |
| `revert` | Reverts a previous commit | Depends |

### Scope

The **scope** is optional and describes what part of the codebase is affected:

**Common Scopes:**
- `hal` - Hardware Abstraction Layer
- `drivers` - Device drivers (display, touch, etc.)
- `protocol` - Binary protocol implementation
- `bus-modes` - Bus mode implementations (i2c, spi, uart, etc.)
- `firmware` - Main application firmware
- `gui` - GUI/Slint related changes
- `mobile` - Mobile app (Flutter/Tauri)
- `docs` - Documentation
- `ci` - CI/CD pipelines

### Subject

The **subject** is a short description (50 characters or less):

- Use imperative mood: "add" not "added" or "adds"
- Don't capitalize first letter
- No period (.) at the end

### Body

The **body** is optional and can contain:

- Detailed explanation of what changed
- Motivation for the change
- Comparison with previous behavior

Wrap at 72 characters per line.

### Footer

The **footer** is optional and can contain:

- **BREAKING CHANGE**: Describes breaking API changes (triggers MAJOR version bump)
- **Closes**: Reference to issue(s) being closed
- **Refs**: Reference to related issue(s)

## Examples

### Feature Addition

```
feat(i2c): add register dump functionality

Implement ability to dump all registers from an I2C device
with a single command. Useful for debugging and reverse
engineering.

Closes #42
```

### Bug Fix

```
fix(display): correct framebuffer alignment

The framebuffer was not properly aligned for DMA transfers,
causing occasional display corruption. Changed allocation to
use 32-byte alignment.

Fixes #138
```

### Breaking Change

```
feat(protocol): change message framing format

BREAKING CHANGE: Protocol version bumped to 2.0

The new framing uses variable-length encoding for the length
field, reducing overhead for small messages. Clients must
update to support the new format.

Migration guide: docs/protocol-v2-migration.md

Closes #201
```

### Documentation

```
docs(rfid): add wiring diagrams for RC522

Include detailed connection diagrams with Mermaid and
purchase links for common RFID reader modules.
```

### Refactoring

```
refactor(hal): extract peripheral initialization

Move peripheral init code from main to separate modules for
better organization and testability. No functional changes.
```

### Multiple Changes

```
feat(gui): implement battery indicator and settings screen

- Add battery voltage monitoring
- Create settings screen with Slint
- Add brightness control slider
- Implement power saving mode toggle

Closes #155, #167
```

## Breaking Changes

A **BREAKING CHANGE** must be indicated in the footer:

```
refactor(protocol)!: remove deprecated commands

BREAKING CHANGE: Removed legacy v1 protocol commands

The following commands have been removed:
- CMD_OLD_SCAN
- CMD_LEGACY_CONFIG

Use the new unified command interface instead. See
docs/migration-guide.md for details.

Closes #203
```

**Note**: The `!` after the type/scope is optional but recommended for visibility.

## Commit Message Template

Create `.gitmessage` in your home directory:

```bash
# <type>(<scope>): <subject>
#
# <body>
#
# <footer>
#
# Type must be one of:
#   feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert
#
# Scope examples:
#   hal, drivers, protocol, bus-modes, firmware, gui, mobile, docs, ci
#
# Subject:
#   - Use imperative mood ("add" not "added")
#   - Don't capitalize first letter
#   - No period at the end
#   - Keep under 50 characters
#
# Body:
#   - Explain what and why (not how)
#   - Wrap at 72 characters
#
# Footer:
#   - Include "BREAKING CHANGE:" for breaking changes
#   - Reference issues: "Closes #123" or "Refs #456"
```

Configure Git to use it:

```bash
git config --global commit.template ~/.gitmessage
```

## Tools

### Commitizen

Install [Commitizen](https://commitizen.github.io/cz-cli/) for interactive commit creation:

```bash
npm install -g commitizen cz-conventional-changelog

# Initialize in project
commitizen init cz-conventional-changelog --save-dev --save-exact
```

Usage:

```bash
git add .
git cz
```

### Commitlint

Install [commitlint](https://commitlint.js.org/) to enforce commit message format:

```bash
npm install --save-dev @commitlint/cli @commitlint/config-conventional
```

Create `commitlint.config.js`:

```javascript
module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'scope-enum': [
      2,
      'always',
      [
        'hal',
        'drivers',
        'protocol',
        'bus-modes',
        'firmware',
        'gui',
        'mobile',
        'docs',
        'ci'
      ]
    ]
  }
};
```

Add to `.git/hooks/commit-msg`:

```bash
#!/bin/sh
npx commitlint --edit $1
```

### Standard Version

Use [standard-version](https://github.com/conventional-changelog/standard-version) for automated releases:

```bash
npm install --save-dev standard-version
```

In `package.json`:

```json
{
  "scripts": {
    "release": "standard-version"
  }
}
```

Usage:

```bash
# Dry run
npm run release -- --dry-run

# Actual release
npm run release

# Release as specific version
npm run release -- --release-as 1.2.0
```

## GitHub Integration

### PR Title Format

Pull Request titles should also follow conventional commits:

```
feat(i2c): add multi-master support
```

### Auto-Labeling

Configure `.github/labeler.yml` to automatically add labels based on commit type:

```yaml
feat:
  - 'feat*'
  
bug:
  - 'fix*'
  
documentation:
  - 'docs*'
```

### Auto-Changelog in PRs

Use [Release Drafter](https://github.com/release-drafter/release-drafter) to generate release notes:

```yaml
# .github/release-drafter.yml
name-template: 'v$RESOLVED_VERSION'
tag-template: 'v$RESOLVED_VERSION'
categories:
  - title: '🚀 Features'
    labels:
      - 'feat'
  - title: '🐛 Bug Fixes'
    labels:
      - 'fix'
  - title: '📚 Documentation'
    labels:
      - 'docs'
```

## Common Mistakes

### ❌ Wrong

```
# Missing type
Updated README with installation instructions

# Wrong mood
Added I2C scanning feature

# Too vague
Fix bug

# Mixed changes without grouping
feat: add UART mode and fix display bug
```

### ✅ Correct

```
docs: add installation instructions to README

feat(i2c): add bus scanning feature

fix(protocol): correct CRC calculation for large payloads

feat(uart): add UART bridge mode
fix(display): correct buffer overflow in draw_text
```

## Workflow

1. **Make changes**: Edit code, test locally
2. **Stage changes**: `git add <files>`
3. **Commit with format**: `git commit` or `git cz`
4. **Push**: `git push`
5. **Create PR**: With conventional commit title
6. **Merge**: Squash and merge with proper format

## Enforcement

All commits must follow this convention. PRs with non-conforming commits will be rejected by CI.

## See Also

- [Official Specification](https://www.conventionalcommits.org/)
- [Angular Convention](https://github.com/angular/angular/blob/main/CONTRIBUTING.md#commit)
- [Contributing Guide](./contributing.md)
- [Code Style](./code-style.md)
