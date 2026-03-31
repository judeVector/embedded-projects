import time

import machine
import network
import rp2
import urequests

# === CONFIGURATION ===
SSID = "WIFI-NAME"
PASSWORD = "WIFI-PASSWORD"
COUNTRY = "NG"  # GB, US,

# Initialize and Reset Wi-Fi Hardware
# rp2.country(COUNTRY)
wlan = network.WLAN(network.STA_IF)
wlan.active(False)
time.sleep(1)
wlan.active(True)
wlan.config(pm=0xA11140)


print(f"Connecting to {SSID}...")
wlan.connect(SSID, PASSWORD)

# Wait for connection with specific error handling
max_wait = 20
while max_wait > 0:
    status = wlan.status()

    if wlan.isconnected():
        break

    if status == -3:
        print("❌ Error: Wrong Password! Check your 'password' variable.")
        break
    elif status == -2:
        print("❌ Error: SSID not found. Is your router on?")
        break
    elif status < 0:
        print(f"❌ Connection failed with status: {status}")
        break

    max_wait -= 1
    print(f"Waiting... (Status: {status})")
    time.sleep(1)


if wlan.isconnected():
    print("✅ Connected successfully!")
    print("Local IP:", wlan.ifconfig()[0])

    try:
        print("Testing internet access...")
        response = urequests.get("http://httpbin.org/ip", timeout=10)
        print("✅ Internet works! Public IP:", response.json()["origin"])
        response.close()
    except Exception as e:
        print("❌ Internet request failed (DNS or Routing issue):", e)
else:
    print(f"\n⚠️ Terminated. Final Status: {wlan.status()}")
    print("Performing a HARD RESET in 3 seconds to clear the Wi-Fi chip...")
    time.sleep(3)
    machine.reset()  # This physically reboots the Pico 2W to clear the Wi-Fi chip
