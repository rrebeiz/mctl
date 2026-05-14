<div align="center">

# mctl

A simple music control CLI written in Rust with Waybar integration support.

mctl provides a lightweight command-line interface for controlling media playback and exposing playback information for status bars such as Waybar.

<img src="resources/screenshot.png" alt="Screenshot of the Now Playing Notifier">
</div>

---

## Overview

This project is a small systems-level utility written in Rust that integrates with the Linux desktop environment.

It monitors the active media player through the MPRIS D-Bus interface and displays real-time “Now Playing” notifications using the system notification service.

---

## Features

* Play / pause media playback
* Skip to next track
* Return to previous track
* Stop playback
* Waybar friendly output
* Fast and lightweight Rust implementation

## Dependencies

* `mpris`


---

## Usage
Usage: mctl [COMMAND]
### Commands
* next Play next song. alias n
* previous Play previous song. alias b
* pause Toggle play / pause. alias p
* stop Play next song. alias n
* json Output metadata information as json. alias j
* waybar Output waybar compatible status. alias w
```

## Waybar Support
### Example waybar custom module config:
```
{
  "custom/mctl": {
    "exec": "mctl w",
    "interval": 2,
    "return-type": "json",
    "format": "{text}",
    "tooltip": "{tooltip}",
    "on-click": "mctl p",
    "on-click-right": "mctl n",
    "on-click-middle": "mctl b"
  }
}
```

### Example css:
```
#custom-mctl {
  margin-right: 10px;
}
```

### Example hyperland bindings (.config/hypr/bindings.conf):
```
# Play / Pause
bind = , XF86AudioPlay, exec, mctl pause
bind = $mainMod, P, exec, mctl pause

# Next track
bind = , XF86AudioNext, exec, mctl next
bind = $mainMod, period, exec, mctl next

# Previous track
bind = , XF86AudioPrev, exec, mctl previous
bind = $mainMod, comma, exec, mctl previous

# Stop playback
bind = , XF86AudioStop, exec, mctl stop
bind = $mainMod SHIFT, P, exec, mctl stop
```

### Installing

### using the make file
```bash
make install          # native
make install-bin      # download and install release binary
```

### uninstalling
```bash
make uninstall          # native
make uninstall-bin      # uninstall release binary
```

### Run in development mode

```bash
cargo run
```

### Build release binary

```bash
cargo build --release
```

### Run binary

```bash
./target/release/<binary_name>
```

### building from source

```bash
cargo install --path .

mkdir -p ~/.config/systemd/user
cp mctl.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable mctl
systemctl --user start mctl
systemctl --user status mctl
```

### if downloading the release version from github
```bash
cp mctl  /usr/local/bin
chmod +x /usr/local/bin/mctl
```
repeat the steps for creating a service entry, but change the path to the binary.

---