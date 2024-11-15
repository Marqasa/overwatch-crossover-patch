# Overwatch 2 CrossOver Patch

> **PLEASE NOTE:** A recent update to Overwatch 2 has broken the game on CrossOver. CodeWeavers are aware and are looking into the issue.

This patch enables Overwatch 2 to run on Apple Silicon macs. It uses the latest
versions of [MoltenVK](https://github.com/KhronosGroup/MoltenVK "MoltenVK") and
[DXVK](https://github.com/Gcenx/DXVK-macOS "DXVK") for macOS.

## Disclaimer

This script is provided "as is" without any warranty of any kind, either express
or implied, including but not limited to the warranties of merchantability,
fitness for a particular purpose, and non-infringement. In no event shall the
author be liable for any damages, including any direct, indirect, special,
incidental, or consequential damages of any kind arising out of or in connection
with the use or performance of this script.

## How to use

1. Install **CrossOver** from https://www.codeweavers.com/crossover
2. In CrossOver, click **+ Install** and search for either **Battle.net Desktop
   App** or **Steam**
3. Install **Overwatch**
4. Download the latest patch from the
   [releases](https://github.com/Marqasa/overwatch-crossover-patch/releases)
   page
5. Open the terminal and navigate to the directory where you downloaded the
   patch (e.g. `cd ~/Downloads`)
6. Run the following command: `chmod +x ./overwatch-crossover-patch`
7. Ctrl + Click the patch file and select "Open" from the menu
8. Click "Open" in the dialog that appears
9. When prompted, enter the name of your Overwatch bottle and game client
10. In CrossOver, right click your Overwatch bottle, go to Settings, and enable
    `DXVK Backend for D3D11`

## DXVK Cache

When the game first launches, you will see a message in the bottom left corner
while DXVK loads. The game will run slowly until this message disappears. I
recommend waiting on the login screen and then go to the practice range to test
performance.

## Mouse Acceleration

You can disable mouse acceleration on macOS Sonoma by going to System Settings >
Mouse > Advanced and checking disable pointer acceleration. On older versions of
macOS, I recommend Linear Mouse: https://linearmouse.app/

## Troubleshooting

If you get an "Operation not permitted" error, you may need to give Terminal
Full Disk Access. You can do this by going to System Settings > Privacy &
Security > Full Disk Access and adding Terminal to the list.

## Known Issues

-   No known issues as of CrossOver 24.0.0 and macOS Sonoma.

-   On versions of CrossOver prior to 24.0.0, the mouse sometimes becomes
    unlocked, requiring you to tab out, then back in to fix it. The best way I
    have found to deal with this is to use borderless windowed mode and
    [reduce motion](https://support.apple.com/en-gb/guide/mac-help/mchlc03f57a1/mac)
    in macOS. Then use `cmd + tab` twice quickly to re-lock the mouse. This issues
    is completely resolved in CrossOver 24.0.0.

-   On versions of macOS prior to Sonoma, the game has many missing UI elements
    making menu navigation difficult. This issue is completely resolved in macOS
    Sonoma.

-   For versions of macOS prior to Sonoma, I have made a simple application for
    editing game settings. It is available here:
    [overwatch-settings](https://github.com/Marqasa/overwatch-settings). You can
    also modify the settings file manually here:
    `~/Documents/Overwatch/Settings/Settings_v0.ini`

-   For versions of macOS prior to Sonoma, I have uploaded a gallery showing how
    the UI should look here: [UI Gallery](https://imgur.com/a/exzsCBi). There is
    also a good video for editing settings. Make the game and video full screen
    then tab between them:
    [YouTube Video](https://www.youtube.com/watch?v=tgS_OGABrGY)
