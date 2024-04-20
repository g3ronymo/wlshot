Screenshot tool for wayland. Gtk4 frontend for 
[grim](https://sr.ht/~emersion/grim/) and 
[slurp](https://github.com/emersion/slurp).

## Dependencies
- https://sr.ht/~emersion/grim/
- https://github.com/emersion/slurp

## Config

${XDG_CONFIG_HOME}/wlshot/wlshot.toml
```
default_dir=/path/to/default/directory
```

## Compile
``cargo build --release`` creates a binary in target/release.
