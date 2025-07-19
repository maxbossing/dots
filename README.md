If you are like me, you probably store your dotfiles in a git repo and symlink them into the right places. However, this 
always takes a considerable amount of time to do on a new system. In the past, I resorted to shell scripts to do my bidding,
but that was often machine-specific, wouldn't allow me to configure multiple machines without significant hassle, and was 
generally a bit _clunky_.

# The solution: `dots`

`dots` is a machine-agnostic, fs-agnostic and structure-agnostic deployer for dotfiles stored in an arbitrary directory. 
Written in rust, it's :sparkles: _blazingly fast_ :sparkles: and easy to use. 

## Installation

To install `dots`, head over to [the releases page](https://github.com/maxbossing/dots/releases/latest) and choose the 
appropriate option for your system. `dots` currently only supports linux, but in glibc and MUSL variants.

To build from source, simply clone and run `cargo build --release`.

## Usage

At the core, `dots` just symlinks arbitrary files and directories as specified by a configuration file. Refer to the [dots.toml](#dotstoml) section for an example.

To deploy a set of files, use `dots deploy`.

If you want to un-deploy a set of dots (removing the links created), you can use `dots unlink`

## dots.toml

By default, `dots` will look for a `dots.toml` file in the current directory.

Example `dots.toml`:
```toml
dots_dir = "/home/anon/dots" # If omitted, defaults to working directory

[[dot]]
src = "nvim" # Resolved against dots_dir
dest = ".config/nvim" # Relative paths are resolved against $HOME

[[dot]]
src = "Xmodmap"
dest = "/etc/X11" # Absolute paths are respected

[[dot]]
src = "tmux.conf"
dest = ".tmux.conf"

[[...]]
```

If you want to use a different file (for example a different machine but based on the same files), pass a path to the executable
```bash
dots deploy --config some_file.dots.toml
```

## Todo
- [x] Allow to "undeploy", eg. to automatically remove symlinks created by `dots`
- [x] Absolute Paths in dest field
- [ ] Windows Support

## License
This is licensed under the [MIT](LICENSE) license.
