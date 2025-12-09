# Code Quality Notes

## Minor Improvements for Future Refactoring

The following items were identified during code review but are considered low-priority improvements that don't affect functionality:

### 1. UART Implementation Duplication

**Files**: `rust/hal/src/peripherals/uart.rs`

The `UartBus0` and `UartBus1` implementations have duplicate code for Read and Write traits.

**Potential Solution**: Use a macro or implement a generic UART wrapper:
```rust
macro_rules! impl_uart_io {
    ($uart:ty) => {
        impl<'d> embedded_io::Read for $uart {
            // ... shared implementation
        }
        
        impl<'d> embedded_io::Write for $uart {
            // ... shared implementation
        }
    };
}
```

**Priority**: Low (code works correctly, just not DRY)

### 2. SPI Implementation Duplication

**Files**: `rust/hal/src/peripherals/spi.rs`

The `SpiBus2` and `SpiBus3` implementations have identical SpiBus trait implementations.

**Potential Solution**: Use a macro similar to UART:
```rust
macro_rules! impl_spi_bus {
    ($spi:ty, $peripheral:ty) => {
        impl<'d> SpiBus for $spi {
            // ... shared implementation
        }
    };
}
```

**Priority**: Low (code works correctly, just not DRY)

### 3. Generic Peripheral Wrapper Pattern

**Future Enhancement**: Consider creating a generic peripheral wrapper pattern that could be reused across I2C, SPI, and UART:

```rust
pub struct PeripheralWrapper<T, C> {
    peripheral: T,
    config: C,
}
```

This would reduce duplication across all peripheral types but would require significant refactoring.

**Priority**: Very Low (enhancement for future major version)

## Notes

- All identified issues are stylistic/maintainability concerns
- Current implementation is functionally correct
- Addressing these would reduce ~100 lines of code
- Refactoring should maintain the same public API
- Consider addressing when adding new peripherals (UART2, I2C1, etc.)

## Recommendation

These improvements can be addressed in a future cleanup PR after:
1. Hardware testing confirms all functionality works
2. Other developers have used the API and confirmed it's stable
3. Adding new peripherals would benefit from shared patterns

The current implementation prioritizes clarity and explicitness over DRY, which is appropriate for initial hardware bringup.
