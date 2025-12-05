# Contributing to ESP32 Bus Pirate

Thank you for your interest in contributing to ESP32 Bus Pirate! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Be respectful, inclusive, and professional in all interactions. We're here to build something awesome together!

## Getting Started

### Prerequisites

- **Rust**: Install nightly toolchain with `rustup install nightly`
- **espup**: Install Xtensa toolchain with `cargo install espup && espup install`
- **mdbook**: For documentation with `cargo install mdbook mdbook-mermaid`
- **Hardware**: Waveshare ESP32-S3-Touch-LCD-2.8 board (for testing)

### Repository Setup

```bash
# Clone the repository
git clone https://github.com/x0f5c3/esp32-bus-pirate.git
cd esp32-bus-pirate

# Check out a feature branch
git checkout -b feature/my-awesome-feature
```

## Development Workflow

### 1. Follow Conventional Commits

All commits MUST follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.

**Format:**
```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `build`: Build system changes
- `ci`: CI/CD changes
- `chore`: Other changes

**Examples:**
```bash
git commit -m "feat(i2c): add multi-master support"
git commit -m "fix(display): correct framebuffer alignment"
git commit -m "docs(rfid): add wiring diagrams"
```

### 2. Write Quality Code

**Rust Code:**
```bash
# Format code
cd rust
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --workspace

# Build firmware
cd firmware
cargo build --release
```

**Documentation:**
```bash
# Build and serve locally
cd book
mdbook serve --open
```

### 3. Create Pull Requests

1. **Push your branch**
   ```bash
   git push origin feature/my-awesome-feature
   ```

2. **Create PR** on GitHub with:
   - **Title**: Following conventional commits format
   - **Description**: Clear explanation of changes
   - **Screenshots**: For UI changes
   - **Testing**: How you tested the changes
   - **Breaking Changes**: If any, clearly documented

3. **Address Review Feedback**
   - Respond to comments
   - Make requested changes
   - Push additional commits (will be squashed)

## Project Structure

```
esp32-bus-pirate/
├── rust/              # Rust workspace
│   ├── hal/          # Hardware abstraction
│   ├── drivers/      # Device drivers
│   ├── protocol/     # Binary protocol
│   ├── bus-modes/    # Protocol implementations
│   └── firmware/     # Main application
├── book/             # Documentation (mdBook)
├── docs/             # Original design documents
└── .github/          # CI/CD workflows
```

## Areas for Contribution

### 🔧 Hardware & Drivers

- **HAL Implementation**: Complete peripheral abstractions
- **Display Driver**: Optimize ST7789 performance
- **Touch Driver**: Improve CST328 reliability
- **Sensor Drivers**: IMU, RTC, audio codec

**Skills needed:** Embedded Rust, hardware protocols, datasheet reading

### 📡 Protocol Modes

- **I²C Mode**: Advanced features (10-bit addressing, sniffing)
- **SPI Mode**: Flash operations, SD card support
- **UART Mode**: Half-duplex, auto-baud detection
- **1-Wire, CAN, etc.**: Implementation from scratch

**Skills needed:** Protocol knowledge, embedded programming

### 🎨 GUI Development

- **Slint UI**: Create screens and widgets
- **Touch Interactions**: Gestures, menus
- **Logic Analyzer View**: Waveform rendering
- **Themes**: Dark/light mode support

**Skills needed:** Slint, UI/UX design, embedded graphics

### 📱 Mobile Apps

- **Flutter App**: iOS/Android application
- **Tauri App**: Cross-platform desktop
- **Protocol Client**: Binary protocol implementation
- **Features**: File transfer, real-time monitoring

**Skills needed:** Flutter/Dart, Tauri/TypeScript, mobile development

### 📚 Documentation

- **User Guides**: How-to articles, tutorials
- **Hardware Guides**: Wiring diagrams, component recommendations
- **API Documentation**: rustdoc comments
- **Examples**: Sample projects and use cases

**Skills needed:** Technical writing, diagramming (Mermaid)

### 🧪 Testing

- **Unit Tests**: Test individual modules
- **Integration Tests**: Test complete workflows
- **Hardware Tests**: On-device validation
- **Fuzzing**: Protocol robustness testing

**Skills needed:** Rust testing, embedded testing frameworks

## Coding Standards

### Rust Style

- **Format**: Use `rustfmt` (configured in `rustfmt.toml`)
- **Lints**: Pass `clippy` with no warnings
- **Naming**: Follow Rust conventions (snake_case, PascalCase)
- **Comments**: Document public APIs with `///`
- **Error Handling**: Use `Result<T, E>`, avoid panics

### Documentation Style

- **Markdown**: Use proper headers, lists, code blocks
- **Diagrams**: Use Mermaid for technical diagrams
- **Links**: Use relative links between pages
- **Examples**: Provide code examples where helpful

### Commit Message Style

```
<type>(<scope>): <subject>

<body explaining what and why, not how>

<footer with issue references>
```

**Bad:**
```
fixed bug
updated code
changes
```

**Good:**
```
fix(i2c): correct clock stretching timeout

The I2C peripheral was not properly handling clock stretching
from slow devices. Increased timeout from 10ms to 100ms and
added retry logic.

Fixes #123
```

## Testing Your Changes

### Local Testing

```bash
# 1. Format and lint
cd rust
cargo fmt --all
cargo clippy --all-targets --all-features

# 2. Run tests
cargo test --workspace

# 3. Build firmware
cd firmware
cargo build --release

# 4. Flash to device (if you have hardware)
espflash flash --monitor target/xtensa-esp32s3-none-elf/release/esp32-bus-pirate

# 5. Test documentation
cd ../../book
mdbook build
mdbook serve --open
```

### CI/CD

All PRs must pass CI checks:
- ✅ Code formatting (`cargo fmt`)
- ✅ Linting (`cargo clippy`)
- ✅ Tests (`cargo test`)
- ✅ Documentation build (`mdbook build`)
- ✅ Firmware build (for ESP32-S3)
- ✅ Conventional commit format

## Release Process

Releases are automated via GitHub Actions when tags are pushed:

```bash
# Create and push tag
git tag -a v1.2.3 -m "Release v1.2.3"
git push origin v1.2.3
```

This triggers:
1. **Changelog generation** (git-cliff)
2. **Firmware build** (all targets)
3. **Documentation build**
4. **GitHub Release creation**
5. **Crates.io publication** (when applicable)

## Communication

- **Issues**: Bug reports, feature requests
- **Discussions**: Questions, ideas, showcase
- **Pull Requests**: Code contributions

## Getting Help

- **Documentation**: This book!
- **Rust Embedded Book**: https://docs.rust-embedded.org/book/
- **esp-rs Docs**: https://esp-rs.github.io/book/
- **Matrix Chat**: https://matrix.to/#/#esp-rs:matrix.org

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

---

Thank you for contributing to ESP32 Bus Pirate! 🚀
