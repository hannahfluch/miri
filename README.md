# Miri
An extension to the Niri window manager to allow for Master layout of windows. Looking to support more layout modes soon


## Installation
Run the install script from the latest release and follow the instructions
```
curl -fsSL https://github.com/MintyDoggo/miri/releases/latest/download/install.sh \
  -o /tmp/miri-install.sh \
  && sh /tmp/miri-install.sh \
  && rm /tmp/miri-install.sh
```

Once installed, the `miri` command will be available. Be sure `~/.local/bin` is in your `PATH`. Most distros include it automatically, but if `miri` isn't found after installing, add this to your `~/.bashrc`:
```sh
export PATH="$HOME/.local/bin:$PATH"
```

Then reload your shell with `source ~/.bashrc`

## Configuration
todo

## Misc
ASCII art for installer
```
╔═══════-□×╗╔═════-□×╗    ╔══════-□×╗╔══════-□×╗
║⠿⠿⠯⠥      ║╚════════╝    ║>_ miri  ║║>_       ║
║⠿⠿⠶⠶⠶⠶⠶⠦⠤ ║╔═════-□×╗    ╚═════════╝╚═════════╝
║⠿⠿⠿⠯⠭⠭⠉⠉  ║╚════════╝    ╔══════-□×╗╔══════-□×╗
║>_ miri   ║╔═════-□×╗    ║>_       ║║>_       ║
╚══════════╝╚════════╝    ╚═════════╝╚═════════╝
```