cmake_minimum_required(VERSION 3.13)

# Pull in Pico SDK (change the path if needed)
include($ENV{PICO_SDK_PATH}/external/pico_sdk_import.cmake)

project(pico2w C CXX ASM)
pico_sdk_init()

add_executable(pico2w
    main.c
)

# Enable USB output (optional but very useful for printf debugging)
pico_enable_stdio_usb(pico2w 1)
pico_enable_stdio_uart(pico2w 0)

# Pull in the required libraries
target_link_libraries(pico2w
    pico_stdlib
    pico_cyw43_arch_none   # "none" because we don't need full Wi-Fi/Lwip here
)

# Create UF2 + map + bin files
pico_add_extra_outputs(pico2w)


#  cmake .. -DPICO_BOARD=pico2_w -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
