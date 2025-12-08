# Task #3: Protocol Tests and USB CDC Transport

**Status**: 🔴 Not Started  
**Assigned to**: `@protocol-cli-developer`  
**Priority**: HIGH  
**Dependencies**: Task #1 (HAL for USB)  
**Estimated time**: 7-10 days  
**Started**: TBD  
**Completed**: TBD

## Description

Add comprehensive testing for the binary protocol and implement USB CDC serial transport.

## Files to Create

- [ ] `rust/protocol/tests/codec_tests.rs`
- [ ] `rust/protocol/tests/integration_tests.rs`
- [ ] `rust/firmware/src/transport/usb_cdc.rs`
- [ ] `rust/firmware/src/transport/mod.rs`
- [ ] `tools/test_client.py`

## Requirements

### Protocol Testing
1. 50+ unit tests for MessageCodec
2. Test all message types
3. CRC validation tests
4. Error handling tests
5. Fuzz testing

### USB CDC
1. USB CDC class implementation
2. Frame transport layer
3. RX/TX circular buffers (1KB each)
4. USB connect/disconnect events
5. Flow control and error recovery

### Python Test Client
1. Message encoding/decoding
2. CRC calculation
3. Command-line interface
4. Interactive mode

## Success Criteria

- ✅ All tests pass (50+ tests)
- ✅ Device enumerates as USB serial
- ✅ Works on Windows, Linux, macOS
- ✅ No data corruption
- ✅ Python client works

## Agent: Start Work

When ready to begin, mention `@protocol-cli-developer` in a comment to this file.
