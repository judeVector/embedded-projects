# MicroPython Projects 🐍

Quick setup guide to get MicroPython running on your Raspberry Pi Pico.

---

## ⚡ Quick Setup

### Flash MicroPython Firmware

Download one of these files based on your board:

- **Pico 2 W:** `RPI_PICO2_W-20251209-v1.27.0.uf2`
- **Pico 2:** `RPI_PICO2-20251209-v1.27.0.uf2`

**Steps:**
1. Connect your Pico to your computer via USB
2. Hold the **BOOTSEL** button while plugging in (or press BOOTSEL, then power on)
3. Your Pico will appear as a USB drive
4. Drag & drop the `.uf2` file onto the Pico drive
5. Wait for the transfer to complete and the Pico to reboot

---

### 2️⃣ Install Thonny

Download [Thonny IDE](https://thonny.org/) – a beginner-friendly Python editor perfect for MicroPython.

---

### 3️⃣ Configure Thonny for MicroPython

1. Open **Thonny**
2. Go to **Tools** → **Options** (or **Preferences** on Mac)
3. Select the **Interpreter** tab
4. Choose **MicroPython (Raspberry Pi Pico)** from the dropdown
5. Click **OK**

---

## 📂 Running the Code

Once Thonny is configured:

1. Open any `.py` file from this folder in Thonny
2. Click the **▶️ Run** button (or press `F5`)
3. The code executes on your Pico!

---

## 💡 Tips

- **Save to Pico:** Use `File` → `Save As` and save to **Pico** (not your computer)

---

## 📚 Resources

- [MicroPython Pico Docs](https://docs.micropython.org/en/latest/rp2/quickref.html)
- [Thonny Documentation](https://thonny.org/)
- [MicroPython Tutorial](https://micropython.org/tutorial/)

---

Happy coding! 🎉
