# Ladder
A small tool to store useful commands and paste them in to clipboard to relieve your brain.
Simply just ladder!

# How it works
Store your commands with 'ladder -a "Description of command" "Command"'
Show all commands with 'ladder' and select the numeric representation + enter
and the command will be pasted into your clipboard


# Note to self
Since I'm developing in linux env, but need this tool for windows aswell, in order to cross compile
to target 'x86_64-pc-windows-gnu' need to create config in '~/.cargo/config' with contents:
```
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
```

Then just fire it off with rust cargo build --target=x86_64-px-windows-gnu


