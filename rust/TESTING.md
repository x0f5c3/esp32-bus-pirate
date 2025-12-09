# Testing Guide

This document describes the testing strategy for the ESP32 Bus Pirate Rust firmware.

## Overview

The project uses a hybrid testing approach:
- **Host-side unit/integration tests**: For hardware-independent logic (protocol, state machines, data structures)
- **Hardware-in-loop tests**: For firmware running on actual ESP32 hardware (future work)

## Running Tests

### Quick Start

```bash
cd rust
./run-tests.sh
```

### Individual Package Tests

```bash
# Protocol tests (message encoding/decoding, CRC validation)
cd rust/protocol
cargo test

# Bus mode tests (when available)
cd rust/bus-modes
cargo test --lib

# Driver tests (when available)
cd rust/drivers
cargo test --lib
```

**Important**: Tests are automatically configured to run on the host (x86_64) target. The `.cargo/config.toml` in the protocol crate overrides the default ESP32 target for test builds.

### CI Testing

Tests run automatically on:
- Push to `main`, `develop`, or `copilot/**` branches
- Pull requests to `main` or `develop`

See `.github/workflows/test.yml` for the CI configuration.

## Test Organization

### Protocol Tests (`protocol/tests/`)

Integration tests for the binary protocol:
- **Message encoding/decoding**: Roundtrip serialization with postcard
- **Frame validation**: Start/end bytes, version checking
- **CRC validation**: Data integrity checks
- **Error handling**: Invalid frames, corrupted data, version mismatches
- **Edge cases**: Large payloads, truncated frames, all message variants

Example:
```rust
#[test]
fn test_message_encoding_all_variants() {
    let msg = Message::SetMode { mode: Mode::I2c };
    let encoded = MessageCodec::encode(&msg).expect("encoding failed");
    let decoded = MessageCodec::decode(&encoded).expect("decoding failed");
    assert_eq!(msg, decoded);
}
```

### Unit Tests (within source files)

Unit tests are defined using `#[cfg(test)]` modules within the codec source:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_crc_validation() {
        // Test CRC corruption detection
    }
}
```

## Test Coverage

Current test coverage by crate:

| Crate | Coverage | Description |
|-------|----------|-------------|
| `protocol` | ✅ Comprehensive | Message encoding, CRC, framing |
| `bus-modes` | 🚧 Planned | Bus protocol state machines |
| `drivers` | 🚧 Planned | Driver abstractions (mockable) |
| `hal` | ⚠️ Hardware-dependent | Board initialization (HiL only) |
| `firmware` | ⚠️ Hardware-dependent | Main application (HiL only) |

Legend:
- ✅ = Tests implemented and passing
- 🚧 = Tests planned but not yet implemented
- ⚠️ = Requires hardware, not suitable for CI

## Writing New Tests

### For `no_std` Crates

The protocol and bus-modes crates are `no_std`, so they need integration tests in a separate `tests/` directory that runs in a `std` environment:

```rust
// protocol/tests/my_test.rs
use esp32_bus_pirate_protocol::*;

#[test]
fn test_something() {
    // Your test code
}
```

### Test Best Practices

1. **Test naming**: Use descriptive names that explain what is being tested
   - ✅ `test_corrupted_crc_detected`
   - ❌ `test1`

2. **Arrange-Act-Assert**: Structure tests clearly
   ```rust
   #[test]
   fn test_example() {
       // Arrange: Set up test data
       let msg = Message::GetMode;
       
       // Act: Perform the operation
       let result = MessageCodec::encode(&msg);
       
       // Assert: Verify the outcome
       assert!(result.is_ok());
   }
   ```

3. **Test independence**: Each test should be self-contained and not depend on others

4. **Edge cases**: Test boundary conditions, empty inputs, maximum sizes

5. **Error paths**: Test both success and failure cases

## Future Testing Plans

### Bus Modes Tests
- I2C state machine transitions
- SPI transfer logic
- UART configuration validation
- Error handling and recovery

### Driver Tests
- Mock hardware interfaces using traits
- Test driver initialization
- Validate register operations
- Test error conditions

### Hardware-in-Loop Tests
- Actual I2C/SPI/UART communication
- Display driver testing
- Touch controller integration
- Full firmware integration tests

## Continuous Integration

The CI pipeline (`test.yml`) runs:
1. **Formatting check**: `cargo fmt --check`
2. **Linting**: `cargo clippy`
3. **Tests**: `cargo test` for each testable crate
4. **Documentation**: `cargo doc` to ensure docs build

All checks must pass before merging PRs.

## Troubleshooting

### "can't find crate for `test`" in `no_std` crates

This happens when you try to add `#[cfg(test)]` modules to `no_std` crates. Solution: Use integration tests in the `tests/` directory instead.

### Tests fail with "esp-hal" errors

The HAL and firmware crates depend on ESP32 hardware. These tests cannot run in CI and should be excluded from the test suite. Only test hardware-independent crates.

### Slow test execution

Tests run faster with:
```bash
cargo test --release  # Use optimized builds
cargo test -- --test-threads=4  # Parallel execution
```

## Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Test Options](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Integration Tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests)
