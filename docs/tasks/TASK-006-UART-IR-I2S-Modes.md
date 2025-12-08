# Task #6: UART, Infrared, and I2S Bus Modes

**Status**: 🔴 Not Started  
**Assigned to**: `@bus-mode-engineer`  
**Priority**: HIGH  
**Dependencies**: Task #1 (HAL), Task #3 (USB CDC)  
**Estimated time**: 14-21 days  
**Started**: TBD  
**Completed**: TBD

## Description

Implement UART, Half-Duplex UART, Infrared, and I2S protocol modes.

## C Reference Files

- `src/Controllers/UartController.cpp`
- `src/Services/UartService.cpp`
- `src/Controllers/HdUartController.cpp`
- `src/Services/HdUartService.cpp`
- `src/Controllers/InfraredController.cpp`
- `src/Services/InfraredService.cpp`
- `src/Controllers/I2sController.cpp`
- `src/Services/I2sService.cpp`

## Files to Create/Modify

- [ ] `rust/bus-modes/src/uart.rs` (expand)
- [ ] `rust/bus-modes/src/hduart.rs` (new)
- [ ] `rust/bus-modes/src/infrared.rs` (new)
- [ ] `rust/bus-modes/src/i2s.rs` (new)

## UART Mode Features

1. All baud rates (9600-921600+)
2. Parity (none, even, odd)
3. Stop bits (1, 1.5, 2)
4. UART bridge mode
5. AT command detection
6. Auto baud detection
7. Line ending options

## HD-UART Mode Features

1. Half-duplex UART communication

## Infrared Mode Features

1. IR transmit via RMT
2. IR receive via RMT
3. 10+ protocols (NEC, RC5, Sony, etc.)
4. Device-B-Gone functionality

## I2S Mode Features

1. Audio playback (PCM5101A)
2. Audio recording
3. Sample rate configuration

## Success Criteria

- ✅ UART bridge works
- ✅ Auto-baud detects rates
- ✅ IR can control TV/AC
- ✅ Audio playback is clear

## Agent: Start Work

When ready to begin, mention `@bus-mode-engineer` in a comment to this file. Requires Task #1 and #3.
