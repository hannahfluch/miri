# Miri
An extension to the Niri window manager to allow for Master layout of windows

https://github.com/user-attachments/assets/b415a86f-6775-40c6-8370-d418a5cf905b

**IMPORTANT:** this project is a work in progress and there are no guarantees of functionality at the moment. **There will be bugs**, so please feel free to report them or make a pull request and I'll do my best to get on it. Master layout mode is currently in a usable state but there are certain edge cases which need to be ironed out

Other than adding more layout modes, I'd like to make a DMS plugin for the bar which shows the current mode of the current workspace on that output

## Supported layout modes
- Master       - In progress
- Grid         - Planned
- Hybrid       - Planned
- Drag and pan - Planned
- Dwindle      - not planned

## Installation and Setup
Run the install script from the latest release and follow the instructions
```sh
curl -fsSL https://github.com/MintyDoggo/miri/releases/latest/download/install.sh \
  -o /tmp/miri-install.sh \
  && sh /tmp/miri-install.sh \
  && rm /tmp/miri-install.sh
```

Once installed, the `miri` command will be available. Be sure `~/.local/bin` is in your `PATH`:
```sh
export PATH="$HOME/.local/bin:$PATH"
```

Then reload your shell with `source ~/.bashrc`

## Configuration
Miri can be configured though `~/.config/miri/config.toml`. These options are not finalized and will likely change, but if you are interested, here is the current configuration list:

```toml
default_workspace_mode = "master"
master_column_default_width_percentage = 50.0
master_maximize_single_window = true
```

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
