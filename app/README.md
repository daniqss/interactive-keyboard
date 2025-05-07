# interactive keyboard app
multiplataform client for interactive keyboard

# development setup
## prerequisites
install tauri [prerequisites](https://v2.tauri.app/es/start/prerequisites/) for your system

```bash
# for archlinux
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
npm run tauri dev
```

# how to build
```bash
npm run tauri build
```
