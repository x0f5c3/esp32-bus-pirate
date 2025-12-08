# Task #4: I2C, 1-Wire, 2-Wire, and 3-Wire Bus Modes

**Status**: 🔴 Not Started  
**Assigned to**: `@bus-mode-engineer`  
**Priority**: HIGH  
**Dependencies**: Task #1 (HAL), Task #3 (USB CDC)  
**Estimated time**: 14-21 days  
**Started**: TBD  
**Completed**: TBD

## Description

Implement I2C, 1-Wire, 2-Wire, and 3-Wire protocol modes.

## C Reference Files

- `src/Controllers/I2cController.cpp`
- `src/Services/I2cService.cpp`
- `src/Controllers/OneWireController.cpp`
- `src/Services/OneWireService.cpp`
- `src/Controllers/TwoWireController.cpp`
- `src/Controllers/ThreeWireController.cpp`

## Files to Create/Modify

- [ ] `rust/bus-modes/src/i2c.rs` (expand)
- [ ] `rust/bus-modes/src/onewire.rs` (new)
- [ ] `rust/bus-modes/src/twowire.rs` (new)
- [ ] `rust/bus-modes/src/threewire.rs` (new)

## I2C Mode Features

1. Bus scan (7-bit and 10-bit)
2. Register read/write
3. Bulk operations
4. I2C sniffer
5. Clock stretching
6. Repeated start
7. EEPROM dump
8. Device identification

## 1-Wire Mode Features

1. Reset and presence detect
2. ROM search algorithm
3. iButton support (DS1990A)
4. DS18B20 temperature sensor
5. EEPROM operations
6. Parasite power mode

## 2-Wire & 3-Wire Features

1. Smart card communication (2-wire)
2. Microwire EEPROM (3-wire)

## Success Criteria

- ✅ I2C scan finds all devices
- ✅ Can read DS18B20 temperature
- ✅ Can dump EEPROM
- ✅ Sniffer captures transactions

## Agent: Start Work

When ready to begin, mention `@bus-mode-engineer` in a comment to this file. Requires Task #1 and #3.
