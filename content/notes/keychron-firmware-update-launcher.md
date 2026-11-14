---
title: "Performing Firmware Updates on Keychron Keyboard"
description: "Guide on performing a Firmware Update on Keychron Keyboard using the Keychron Launcher."
categories: [keychron, firmware, update, wireless]
icon: git
date: 2025-04-15
preview_image_url: "https://images.unsplash.com/photo-1638909067462-1310c4011610?q=1&w=3270&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
published: true
---

Recently I had to update the firmware on a Keychron K3 Max keyboard, while reading on the Keychron
site for Firmware Updates, I found out that theres a Keychron Launcher (which is still in beta),
which allows you to perform the firmware update and also customize the keyboard.

Theres no need to install software, nor drivers, the Keychron Launcher is a web application
that runs on your browser, and it works on both Windows and macOS.

The Keychron Launcher is available on [launcher.keychron.com](https://launcher.keychron.com/#/keymap),
you will just need the included wire and a couple minutes to perform the update.

## Updating Keyboard Firmware

Keychron Keyboards are highly customizable, you can setup diffent layouts, keymaps, and also
backlight behavior.

Plug your keyboard using the included USB-C wire, and open the Keychron Launcher.

Then click on the "Connect +" button on the top-left corner of the window, and select your keyboard
from the pop-up menu.

![](/notes/images/keychron-firmware-update-launcher/01-connecting.gif)

Once the keyboard is connected, you will see a map of your keyboard and some options on the left
navigation bar.

![](/notes/images/keychron-firmware-update-launcher/02-firmware-update.gif)

Perform the firmware update by clicking on the "Firmware Update" option on the left navigation bar,
the current version and the most recent version will be displayed.

### Perform the Firmware Update

You will need to connect to the Keychron Launcher using the Booting Mode, to do this, follow the
instuctions displayed on the screen.

For the Keychron K3 Max, you will need to:

1. Disconnect the keyboard from the computer.
2. Press and hold the "ESC" key while plugging the keyboard back to the computer.
3. Choose tho BOOTLOADER Hardware detected from the pop-up menu.
4. Press the `Match Device` button.

For reference, here is an screenshot on the original instructions:

![Bootloader Mode](/notes/images/keychron-firmware-update-launcher/instructions.png)

The `Flash Firmware` button will be enabled now, click on it to start the update process.

> While the Firmware is being _Flashed_, do not use the keyboard nor disconnect it from the computer.

![](/notes/images/keychron-firmware-update-launcher/03-firmware-transfer.gif)

## Updating the Wireless Firmware

Both the 2.4 GHz and Bluetooth firmware can be updated using the Keychron Launcher.
A different option is available on the left navigation bar, just click on the "Wireless Firmware".

![](/notes/images/keychron-firmware-update-launcher/04-wireless-firmware.gif)

Flashing the wireless firmware is much more straightforward, just click on the `Next` button and
then on the `Flash Firmware` button.

Once ready a summary is printed, both fields: `Current version` and `Latest version` should
be the same.

![](/notes/images/keychron-firmware-update-launcher/05-post-upgrade.png)
