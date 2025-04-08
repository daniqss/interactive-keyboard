# interactive keyboard app
multiplataform client for interactive keyboard

# development setup
## prerequisites
install rust, bun and tauri requirements

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

# with chaotic aur
sudo pacman -S --needed bun
# or with curl -fsSL https://bun.sh/install | bash

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

# run website only
```bash
docker build -f Dockerfile -t interactive-keyboard-web .
 docker run -p 80:80 interactive-keyboard-web:latest
```