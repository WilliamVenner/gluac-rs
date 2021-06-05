# 👨‍💻 gluac-rs

Compile Garry's Mod Lua into bytecode using Rust!

## Features

* Compatible with Windows and Linux
* Works with 32-bit and 64-bit branches of the game (you must compile for the correct target however)
* Thread-safe

## Usage

```rust
// A utility macro for user-friendly generation of Lua-compatible CStrings.
use gluac::lua_string;

// The instance of our bytecode compiler. This internally creates and prepares a Lua state and closes it when dropped.
let compiler: BytecodeCompiler = gluac::compiler();

// Compiling a Lua source code string
let result: Result<Vec<u8>, BytecodeError> = compiler.compile_string(lua_string!(r#"print("Hello, world!")"#));
...

// Compiling a file
let result: Result<Vec<u8>, BytecodeError> = compiler.compile_file(lua_string!(r#"path/to/file.lua"#));
...
```

## Dependencies

This crate requires a few dependencies to be in the same directory as the executable.

### Where to find them

You can find these libraries in your Garry's Mod installation, they appear to pop up in a number of different paths on different platforms and branches, so here's all the ones I know of:

* `bin/`
* `bin/win64`
* `bin/linux32`
* `bin/linux64`
* `garrysmod/bin`

Take care to use the correct dependencies for your target branch of the game (32-bit/64-bit)

### Windows

* `lua_shared.dll`
* `tier0.dll`
* `vstdlib.dll`

### Linux

You may need to add the directory to the `LD_LIBRARY_PATH` environment variable.

* `lua_shared.so`
* `libtier0.so`
* `libvstdlib.so`

I think older Garry's Mod versions have `_srv` suffixes in the file names for these libraries. These are also supported.

## Credits

[Willox](https://github.com/willox) - base code for Lua bindings and lua_shared loading

[Mats](https://github.com/m4tsa) - helping :D
