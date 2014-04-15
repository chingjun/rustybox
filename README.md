# Rustybox
Rustybox is something like busybox, but written in Rust language.

# How to build
To compile, run
```
rustc rustybox.rs
```
and an executable named `rustybox` will be generated in the current directory.

To run a tool, just execute
```
./rustybox cal
```

Or, you can create a symbolic link to rustybox
```
ln -s rustybox cal
./cal
```

# License
Rustybox is released under MIT license
