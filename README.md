# gaspi

![gaspi hunting](chasse-au-gaspi.jpg)

## Dependencies

- https://crates.io/crates/sysinfo


```bash
cat << EOF > /gaspi/.cargo/config
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF
```

```bash
rustup target add armv7-unknown-linux-gnueabihf
cargo build --target=armv7-unknown-linux-gnueabihf
```

