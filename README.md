# RustyGL

## Installation

Start by initializing submodules:

```sh
git submodule update --init --recursive --depth 1
```

### Linux

If you are using Ubuntu install the following components:

```sh
apt-get install -qy libglfw3-dev libgl1-mesa-dev
```

### MacOS

If you are using brew run the following command to install required components:

```sh
brew install glfw
```

### Windows

Please use WSL for now and refer to [Linux](#Linux)

## Usage

To use this program you have to first obtain an obj file. You then run

```sh
rusty_gl <path_to_obj_file>
```

Or from source

```bash
cargo run -r -- <path_to_obj_file>
```
