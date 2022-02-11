
# hex-rs

A simple command line hex viewer written in Rust.


## Usage/Examples

- Read the contents of a file by passing a valid file path
    ![](https://i.imgur.com/nbSN8Jt.png)
- Read a simple string
    ![](https://i.imgur.com/DTwbqoi.png)
- Read contents from pipe
    ![](https://i.imgur.com/aLx8Dq7.png)
## Installation

Installation with AUR

```bash
git clone https://aur.archlinux.org/hex-rs-bin.git && cd hex-rs
makepkg -si
```

If you use [paru](https://github.com/Morganamilo/paru) or [yay](https://github.com/Jguer/yay):

```bash
paru hex-rs-bin
# or
yay hex-rs-bin
```