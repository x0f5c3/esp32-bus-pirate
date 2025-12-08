# Task #2: Implement Display and Touch Drivers

**Status**: 🔴 Not Started  
**Assigned to**: `@display-touch-developer`  
**Priority**: HIGH  
**Dependencies**: Task #1 (HAL must be complete)  
**Estimated time**: 10-14 days  
**Started**: TBD  
**Completed**: TBD

## Description

Integrate ST7789 display driver and CST328 touch controller driver for the Waveshare ESP32-S3-Touch-LCD-2.8 board.

## Files to Implement

- [ ] `rust/drivers/src/display/st7789.rs`
- [ ] `rust/drivers/src/display/mod.rs`
- [ ] `rust/drivers/src/touch/cst328.rs`
- [ ] `rust/drivers/src/touch/mod.rs`
- [ ] `rust/firmware/examples/display_demo.rs`
- [ ] `rust/firmware/examples/touch_demo.rs`

## Requirements

### Display (ST7789)
1. Use `st7789` crate + `embedded-graphics`
2. Initialize as 240×320 portrait mode
3. Implement `DrawTarget` trait
4. PWM backlight control (0-100%)
5. Framebuffer in PSRAM if available
6. Double buffering

### Touch (CST328)
1. I2C driver at address 0x5A
2. Read touch coordinates (X, Y)
3. Detect events: press, release, move
4. Multi-touch support (up to 5 points)
5. Interrupt-driven via GPIO4
6. Calibration routine

## Success Criteria

- ✅ Display shows clear text
- ✅ Backlight adjustable
- ✅ Frame rate >10 FPS
- ✅ Touch accurate
- ✅ No ghost touches
- ✅ Multi-touch works

## Agent: Start Work

When ready to begin, mention `@display-touch-developer` in a comment to this file. Requires Task #1 completion.
