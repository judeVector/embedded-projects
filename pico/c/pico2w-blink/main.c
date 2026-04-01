#include "pico/stdlib.h"
#include "pico/cyw43_arch.h"
#include <stdbool.h>

int main() {
    // Initialize the CYW43 architecture (required for the wireless chip / LED)
    if (cyw43_arch_init()) {
        // Failed to initialize — usually means something is wrong with the SDK setup
        return -1;
    }

    while (true) {
        // Turn LED on
        cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, 1);
        sleep_ms(1000);   // on for 1 s

        // Turn LED off
        cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, 0);
        sleep_ms(1000);   // off for 1 s
    }
}
