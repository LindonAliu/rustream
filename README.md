# Rustream
A Rust project that reads streams from an M3U file, and maybe more one day

## Prerequisites
If you are on Windows or use the flatpak on Linux; SKIP THIS PART.

The app depends on mpv, ffmpeg and yt-dlp.
If you are on MacOS, you must use Brew or MacPorts to install those dependencies.

On Fedora, you must add rpmfusion to install those packages.

On Debian or LTS distro, I would strongly suggest using a backport for yt-dlp.

The Windows build **comes with mpv included** (.msi), but you can still install mpv from a package manager of your choice to always have the latest version installed

```
brew install mpv ffmpeg yt-dlp #MacOS
sudo dnf install mpv ffmpeg yt-dlp #Fedora
sudo zypper install mpv ffmpeg yt-dlp #OpenSUSE
sudo pacman -Syu mpv ffmpeg yt-dlp #Arch
sudo apt install mpv ffmpeg yt-dlp #Debian/Ubuntu
scoop install mpv ffmpeg yt-dlp # Windows
choco install mpv ffmpeg yt-dlp # Windows alternative
```
