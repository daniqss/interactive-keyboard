# interactive keyboard app
multiplataform client for interactive keyboard

# development setup
## prerequisites
install rust, node and tauri requirements

```bash
sudo pacman -Syu
sudo pacman -S --needed \
  webkit2gtk-4.1 \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  libappindicator-gtk3 \
  librsvg

sudo pacman -S --needed nodejs npm 
sudo pacman -S --needed rust
```

## how to run 
```bash
bun run tauri
```

# how to build
```bash
bun tauri build
```
