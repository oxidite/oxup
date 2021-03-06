# Oxup

Oxup is a tool for managing installations and packages of oxido.

## Installation

### Windows

```sh
wget https://github.com/oxidite/oxup/releases/latest/download/oxup-windows.zip
unzip oxup-windows.zip
oxup.exe setup
setx PATH "C:\oxido;%PATH%"
```

### Linux

```bash
wget https://github.com/oxidite/oxup/releases/latest/download/oxup-linux.tar.gz
tar -xf oxup-linux.tar.gz
./oxup setup
```
#### Bash
```bash
echo '. "$HOME/.oxido/env"' >> $HOME/.bashrc
```

#### Zsh
```zsh
echo '. "$HOME/.oxido/env"' >> $HOME/.zshrc
```

### Macos

```bash
wget https://github.com/oxidite/oxup/releases/latest/download/oxup-darwin.zip
unzip oxup-darwin.zip
./oxup setup
```
#### Bash
```bash
echo '. "$HOME/.oxido/env"' >> $HOME/.bashrc
```

#### Zsh
```zsh
echo '. "$HOME/.oxido/env"' >> $HOME/.zshrc
```

Then you can restart your shell and use oxido.

## Usage

```bash
oxup <command> [OPTIONS]
```

## Commands

- `add` add packages to your project
- `install` install oxido interpreter
- `remove` remove packages from your project
- `update` update the oxido interpreter to the latest version permitted by semver
- `uninstall` uninstall oxido interpreter
- `version` prints the version

Options:
- `-W` force run as windows
- `-L` force run as linux
- `-M` force run as macos
