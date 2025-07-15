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

## dots.toml

At the core, `dots` just symlinks arbitrary files and directories as specified by a configuration file. By default, `dots`
will look for a `dots.toml` file in the current directory.

Example `dots.toml`:
```toml
dots_dir = "/home/anon/dots" # If omitted, defaults to working directory

[[dot]]
src = "nvim" # Resolved against dots_dir
dest = ".config/nvim" # Resolved against $HOME

[[dot]]
src = "fish"
dest = ".config/fish"

[[dot]]
src = "tmux.conf"
dest = ".tmux.conf"

[[...]]
```

If you want to use a different file (for example a different machine but based on the same files), pass a path to the executable
```bash
dots some_file.dots.toml
```

## License
This is licensed under the [MIT](LICENSE) license.