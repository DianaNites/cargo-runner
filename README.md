# Cargo-runner

Tool to help with the `.cargo/config` `target.$triple.runner` field.

Primarily intended for use with `cargo-sysroot` and `cargo-image`,
so defaults to running `qemu-system-x86_64 -drive format=raw,file=FILE`

## Usage

In `.cargo/config`

```toml
[target.$triple]
runner = "cargo runner"
```

In `Cargo.toml`

```toml
[package.metadata.cargo-runner]
with_suffix = ".bin"
```
