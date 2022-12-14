# Bloody.rs - a small utility to control bloody mouse backlight intensity

* CLI and tray mode supported
* Tested with [AL90 Mouse](http://www.bloody.com/en/product.php?pid=10&id=100)

# Compilation

```bash
git clone git@github.com:encrawler/bloody.rs.git
cd bloody.rs
cargo run
```

# Install

```
cargo install --path .
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
```

# USB device access fix (linux)

* By default, you need to be root to access USB devices
* Create a new udev rule to drop this requirement for bloody mouses:

```bash
echo 'SUBSYSTEM=="usb", ATTRS{idVendor}=="09da", MODE="0666"' | sudo tee /etc/udev/rules.d/a4.rules
# Replug the mouse to reapply new udev settings
```

# Screenshots
> **Note:** If you don't see tray icons on gnome, ensure you have [AppIndicator extension](https://extensions.gnome.org/extension/615/appindicator-support/) enabled

![screenshot1](./assets/screenshot1.png)
![screenshot2](./assets/screenshot2.png)
