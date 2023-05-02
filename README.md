# Overwatch 2 CrossOver Patch

This patch enables Overwatch 2 to run on Apple silicon Macs (M1 & M2). It uses
the latest versions of
[MoltenVK](https://github.com/The-Wineskin-Project/MoltenVK/releases 'MoltenVK')
and [DXVK](https://github.com/Gcenx/DXVK-macOS/releases 'DXVK') for macOS.

## Disclaimer

This script is provided "as is" without any warranty of any kind, either express
or implied, including but not limited to the warranties of merchantability,
fitness for a particular purpose, and non-infringement. In no event shall the
author be liable for any damages, including any direct, indirect, special,
incidental, or consequential damages of any kind arising out of or in connection
with the use or performance of this script.

## How to use

1. Install **CrossOver** from https://www.codeweavers.com/crossover
2. In CrossOver, click **+ Install** and search for **Battle.net Desktop App**
3. Install **Overwatch**
4. Download the latest patch from the releases page
5. Open the terminal and navigate to the directory where you downloaded the
   patch (e.g. `cd ~/Downloads`)
6. Run the following command: `chmod +x ./overwatch-crossover-patch`
7. Ctrl + Click the patch file and select "Open" from the menu
8. Click "Open" in the dialog that appears
9. When prompted, enter the name of your Overwatch bottle
10. In CrossOver, right click your Overwatch bottle, go to Settings, and enable
    `DXVK Backend for D3D11`

## DXVK Cache

When the game first launches, you will see a message in the bottom left corner
while DXVK loads. The game will run slowly until this message disappears. I
recommend waiting on the login screen and then go to the practice range to test
performance.

## Settings

I have made a simple application for editing game settings. It is available
here: https://github.com/Marqasa/overwatch-settings

You can also modify the settings file manually here:
~/Documents/Overwatch/Settings/Settings_v0.ini

## Menu Navigation

To help with menu navigation I have uploaded a gallery showing how the UI should
look here: https://imgur.com/a/exzsCBi

There is also a good video here for editing settings. Make the game and video
full screen then tab between them: https://www.youtube.com/watch?v=tgS_OGABrGY

## Mouse Acceleration

To disable mouse acceleration on macOS, I recommend Linear Mouse:
https://linearmouse.app/

## Known Issues

Menus have a lot of invisible elements. It is currently very difficult to change
settings/navigate the menus (you have to know where everything is positioned)

The mouse sometimes becomes unlocked, requiring you to tab out, then back in to
fix it. The best way I have found to deal with this is to use borderless
windowed mode,
[reduce motion](https://support.apple.com/en-gb/guide/mac-help/mchlc03f57a1/mac)
in macOS, and cmd + tab twice quickly to re-lock the mouse.
