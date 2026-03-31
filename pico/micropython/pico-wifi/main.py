import time

import machine
import network
import rp2

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

# Start Connection
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

# Final Result
if wlan.isconnected():
    print("✅ Connected successfully!")
    ip_info = wlan.ifconfig()
    print("IP address    :", ip_info[0])
    print("Subnet Mask   :", ip_info[1])
    print("Gateway/Router:", ip_info[2])
    print("DNS Server    :", ip_info[3])
else:
    print(f"\n⚠️ Terminated. Final Status: {wlan.status()}")
    print("Performing a HARD RESET in 3 seconds to clear the Wi-Fi chip...")
    time.sleep(3)
    machine.reset()
