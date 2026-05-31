<!--
 Copyright (C) 2026 Lordseriouspig
 
 This file is part of starcli.
 
 starcli is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.
 
 starcli is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.
 
 You should have received a copy of the GNU General Public License
 along with starcli.  If not, see <https://www.gnu.org/licenses/>.
-->

Welcome to StarCLI! The following page provides instructions on how to get started using stardance!
If you haven't done so yet, please follow the [Installation Instructions](./installation.md). Once you have installed your copy of StarCLI, come back here so we can get you started.

Great! So you've installed StarCLI, what next?
The first thing you need to do is open your terminal of choice. Once you have done so, simply type in ``star`` and hit enter.
Here you'll see all the commands you can use with StarCLI, along with their options. I've included documentation everywhere I can here, using the ``--help`` option, but if you still need help, you can check out the other pages of the docs here.

{% hint style="warning" %}
If you get an error similar to ``"The term 'star' is not recognized as the name of a cmdlet, function, script file, or operable program."``, you do not have stardance in your PATH variable. To add it, please follow [this guide](./add_to_path.md).
{% endhint %}

![An image of the terminal print of the base usage](https://hc-cdn.hel1.your-objectstorage.com/s/v3/dc74c96bb619983c_image.png)

If you want to understand a bit more about how commands work, take a look at [Commands Basics](./commands/basics/README.md)


## Getting your API Key
Now, before going any further, we need to get our API key to allow StarCLI to interact with Stardance on your behalf.
To do this, follow these steps:
1. Head over to https://stardance.hackclub.com/
2. Click the gear icon near your name in the sidebar

![Press the gear icon next to log our and cookies in the sidebar](https://hc-cdn.hel1.your-objectstorage.com/s/v3/cf6d791565750c61_image.png)

3. Scroll down on that menu and copy your API key.

![Copy your API key in the settings modal](https://hc-cdn.hel1.your-objectstorage.com/s/v3/79dd5ba8da854bef_image.png)

{% hint style="danger" %}
Make sure you keep this token secret! If someone you do not trust gains access to this token, they will be able to access your **entire Stardance account**. If you believe someone has gained access to your token, please regenerate it with the refresh button in the settings menu. You will have to follow the following steps again to add it back into StarCLI.
{% endhint %}

## Adding your token to StarCLI
Adding your token to StarCLI is very simple. All you have to do is run [``star auth set <TOKEN>``](./commands/auth/set.md), with ``<TOKEN>`` being replaced with your actual token.
You may delete your token from StarCLI at any time with [``star auth delete``](./commands/auth/delete.md).

{% hint style="success" %}
StarCLI will store your token securely on your device using your device's keychain. It will **never** store your token externally of your device, and only uses it for authenticating with API requests that you explicitly run.
{% endhint %}

## Next Steps
Congratulations! You have now fully installed and set up StarCLI! From here, you may check out instructions on [how commands work](./commands/basics/README.md), or check out instructions on individual commands. If you ever find yourself stuck, feel free to check out these docs or use the ``--help`` option.