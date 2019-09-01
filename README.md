# Cargo-runner

Tool to help with the `.cargo/config` `target.$triple.runner` field.

Primarily intended for use with `cargo-sysroot` and `cargo-image`.

## Install

Run `cargo install cargo-runner`.

## Usage

In `.cargo/config`

```toml
[target.$triple]
runner = "cargo runner"
```

In `Cargo.toml`

```toml
[package.metadata.cargo-runner]
# The string `$TARGET_FILE` will be replaced with the path from cargo.
command = [
    "qemu-system-x86_64",
    "-drive",
    "format=raw,file=$TARGET_FILE"
]

# Add `.bin` to the `$TARGET_FILE` given by cargo.
suffix = ".bin"
```
