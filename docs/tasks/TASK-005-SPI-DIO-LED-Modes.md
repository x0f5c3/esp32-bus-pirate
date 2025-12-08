# Task #5: SPI, DIO, and LED Bus Modes

**Status**: 🔴 Not Started  
**Assigned to**: `@bus-mode-engineer`  
**Priority**: HIGH  
**Dependencies**: Task #1 (HAL), Task #3 (USB CDC)  
**Estimated time**: 14-21 days  
**Started**: TBD  
**Completed**: TBD

## Description

Implement SPI, DIO (GPIO), and LED protocol modes.

## C Reference Files

- `src/Controllers/SpiController.cpp`
- `src/Services/SpiService.cpp`
- `src/Controllers/DioController.cpp`
- `src/Controllers/LedController.cpp`
- `src/Services/LedService.cpp`

## Files to Create/Modify

- [ ] `rust/bus-modes/src/spi.rs` (expand)
- [ ] `rust/bus-modes/src/dio.rs` (new)
- [ ] `rust/bus-modes/src/led.rs` (new)

## SPI Mode Features

1. All SPI modes (0-3)
2. Full-duplex transfers
3. Flash ID reading (JEDEC)
4. Flash erase/program
5. SD card operations
6. SPI slave mode
7. EEPROM operations (25-series)

## DIO Mode Features

1. GPIO read/write/toggle
2. Pull-up/pull-down config
3. PWM output (frequency, duty)
4. Pulse measurement
5. Servo control

## LED Mode Features

1. WS2812 (NeoPixel) via RMT
2. APA102 (DotStar) via RMT
3. LED animations
4. RGB color control

## Success Criteria

- ✅ Can read SPI flash ID
- ✅ SD card R/W works
- ✅ GPIO PWM works
- ✅ WS2812 LEDs display correctly

## Agent: Start Work

When ready to begin, mention `@bus-mode-engineer` in a comment to this file. Requires Task #1 and #3.
