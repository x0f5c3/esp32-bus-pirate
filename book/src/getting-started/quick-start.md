# Quick Start Guide

Get your ESP32 Bus Pirate up and running in under 10 minutes!

## Prerequisites

- **Waveshare ESP32-S3-Touch-LCD-2.8** board
- USB-C cable (data cable, not charge-only)
- Computer with Chrome/Edge browser (for web flasher)

## Step 1: Flash the Firmware

### Option A: Web Flasher (Recommended)

1. Open the [ESP32 Bus Pirate Web Flasher](https://x0f5c3.github.io/esp32-bus-pirate/webflasher/) in Chrome or Edge
2. Connect your ESP32-S3 board via USB-C
3. Click "Connect" and select your device from the list
4. Click "Install ESP32 Bus Pirate"
5. Wait for the installation to complete (~2 minutes)

### Option B: Manual Flash (Advanced)

```bash
# Install esptool
pip install esptool

# Download firmware
wget https://github.com/x0f5c3/esp32-bus-pirate/releases/latest/download/firmware.bin

# Flash (replace /dev/ttyUSB0 with your port)
esptool.py --chip esp32s3 --port /dev/ttyUSB0 write_flash 0x0 firmware.bin
```

## Step 2: First Boot

After flashing, the device will automatically reboot. You'll see:

1. **Boot logo** on the 2.8" display
2. **Mode selection** menu (Serial, Wi-Fi, or Standalone)
3. **Main menu** with protocol options

## Step 3: Connect via Serial

### Windows

1. Open Device Manager → Ports (COM & LPT)
2. Find "USB Serial Device (COMx)"
3. Open PuTTY or any serial terminal
4. Settings: 115200 baud, 8N1, no flow control
5. Connect and you'll see the prompt: `Bus Pirate>`

### Linux/macOS

```bash
# Find the device
ls /dev/tty* | grep -i usb

# Connect with screen
screen /dev/ttyUSB0 115200

# Or with minicom
minicom -D /dev/ttyUSB0 -b 115200
```

## Step 4: Try Your First Command

At the `Bus Pirate>` prompt, try:

```
> help
Available commands:
  mode     - Select protocol mode
  scan     - Scan for devices
  help     - Show this help
  version  - Show firmware version
  
> version
ESP32 Bus Pirate v1.0.0-rust
Hardware: Waveshare ESP32-S3-Touch-LCD-2.8
```

## Step 5: Select a Mode

Let's try I²C mode:

```
> mode i2c
I²C mode selected
Configure: SDA=GPIO1, SCL=GPIO3
Default frequency: 100kHz

> scan
Scanning I²C bus...
Found 2 devices:
  0x5A (CST328 Touch Controller)
  0x6B (QMI8658C IMU)
```

## What's Next?

- **Connect Hardware**: Check [Hardware Setup](../hardware/waveshare-s3.md)
- **Learn Modes**: Explore [Bus Modes](../modes/i2c.md)
- **Mobile App**: Use [Mobile App](../mobile/overview.md) for wireless control
- **Battery Power**: See [Battery Operation](../hardware/battery.md)

## Troubleshooting

### Device Not Detected

- Try a different USB cable (must support data transfer)
- Hold BOOT button while connecting
- Check drivers: [Installation Guide](./installation.md)

### No Serial Output

- Verify baud rate is 115200
- Check correct COM port/device
- Reset the board (press RST button)

### Display Not Working

- Screen may be off due to power saving
- Touch the display to wake it up
- Adjust brightness in settings

## Getting Help

- **Documentation**: This book!
- **Issues**: [GitHub Issues](https://github.com/x0f5c3/esp32-bus-pirate/issues)
- **Discussions**: [GitHub Discussions](https://github.com/x0f5c3/esp32-bus-pirate/discussions)

---

Ready to dive deeper? Check out the [User Guide](../user-guide/overview.md)!
