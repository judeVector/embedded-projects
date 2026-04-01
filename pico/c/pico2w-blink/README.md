## Pico 2 W LED Blink 💡

A simple C project that blinks the onboard LED on the **Raspberry Pi Pico 2 W** using the CYW43 wireless chip.

---

## 📋 Requirements

Before you begin, make sure you have:

- **Raspberry Pi Pico 2 W** (with USB cable)
- **Pico SDK** installed and `PICO_SDK_PATH` environment variable set
- **ARM GCC Toolchain** (for cross-compilation)
- **CMake** (≥ 3.13)
- **Python 3** (for build scripts)

### Installation (if not already done)

**macOS / Linux:**
```bash
# Install dependencies
sudo apt-get install cmake gcc-arm-none-eabi libnewlib-arm-none-eabi build-essential

# Clone Pico SDK
git clone https://github.com/raspberrypi/pico-sdk.git
cd pico-sdk
export PICO_SDK_PATH=$(pwd)
```

**Windows:**
- Download [ARM Embedded Toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)
- Download [CMake](https://cmake.org/download/)
- Clone [Pico SDK](https://github.com/raspberrypi/pico-sdk.git)
- Set `PICO_SDK_PATH` environment variable to your SDK location

---

## 🔨 Build Instructions

### Step 1: Create a Build Directory

```bash
mkdir build
cd build
```

### Step 2: Generate Build Files with CMake

```bash
cmake .. -DPICO_BOARD=pico2_w -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
```

If `PICO_SDK_PATH` is not set, you can specify it:
```bash
cmake .. -DPICO_SDK_PATH=/path/to/pico-sdk -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
```

### Step 3: Compile the Project

```bash
make
```

If compilation succeeds, you'll see:
```
[100%] Built target pico2w
```

You should now have these files in the `build/` folder:
- `pico2w.uf2` ← **This is what you need to flash!**
- `pico2w.elf`
- `pico2w.bin`
- `pico2w.hex`

---

## ⚡ Flash to Pico 2 W

### Option 1: Drag & Drop (Easiest)

1. **Connect your Pico 2 W** to your computer via USB
2. **Hold the BOOTSEL button** while plugging in (or press BOOTSEL then power on)
3. Your Pico will appear as a USB drive named `RPI-RP2`
4. **Drag & drop** `build/pico2w.uf2` onto the drive
5. The Pico will reboot automatically — **LED should start blinking! 🎉**

### Option 2: Using `picotool`

```bash
# Install picotool first
pip install picotool

# Flash the firmware
picotool load build/pico2w.uf2
```

### Option 3: Using `probe-rs`

```bash
# Install probe-rs
cargo install probe-rs-cli

# Flash the firmware
probe-rs run --chip RP2350 build/pico2w.elf
```

---

## 🔍 Verify It Works

Once flashed:
- The **onboard LED** on your Pico 2 W should **blink on/off** every second
- If nothing happens, check:
  - USB connection
  - Correct `.uf2` file was flashed
  - Try holding BOOTSEL and flashing again

---

**`CMakeLists.txt`** – Build configuration

- Sets project name to `pico2w`
- Links against `pico_stdlib` and `pico_cyw43_arch_none`
- Enables USB output for debugging (optional)
- Generates `.uf2`, `.bin`, `.hex`, and `.map` files

---

## 🛠️ Troubleshooting

### CMake fails with "Could not find Pico SDK"
```bash
# Make sure PICO_SDK_PATH is set
export PICO_SDK_PATH=/path/to/pico-sdk

# Then try again
cmake .. -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
```

### Compilation errors about missing headers
- Ensure your Pico SDK is up to date: `cd pico-sdk && git pull`
- Verify ARM GCC is installed: `arm-none-eabi-gcc --version`

### LED doesn't blink after flashing
- Try holding **BOOTSEL** and power-cycling
- Verify the correct `.uf2` file was used
- Check USB connection
- Try flashing again with drag & drop

---

## 📚 Next Steps

- Modify the timing (change `1000` to different values)
- Add button input to control the LED
- Combine with WiFi features (Pico 2 W)
- Read the official [Pico C SDK docs](https://datasheets.raspberrypi.com/pico/raspberry-pi-pico-c-sdk.pdf)

---

## 📖 Useful Resources

- [Raspberry Pi Pico Documentation](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html)
- [Pico C SDK GitHub](https://github.com/raspberrypi/pico-sdk)
- [Pico Datasheet](https://datasheets.raspberrypi.com/pico/pico_datasheet.pdf)

---

**Happy coding! 🚀**
