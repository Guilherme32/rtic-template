# RTIC Template

This is my personal template to use for future embedded projects. It is based
on the cortex-m-quickstart (https://github.com/rust-embedded/cortex-m-quickstart).
It is basically just that, cleaned up to only have the options I need when
using the black pill v1.2 (STM32F401CCU6), with the dependencies and main.rs
updated to work with RTIC.

Cleaning up, I deleted a whole lot of comments explaining the configurations.
For reference of that, go to the original template.

Also, a blink example using a hardware timer is included to serve as a small
reference to the workflow of the framework.

## Stuff for me to remember when I use it after a while

### Dependencies

To install the dependencies necessary, assuming rust was already installed
using rustup:

```cargo install cargo-generate```
```rustup target add thumbv7em-none-eabihf```
```cargo install cargo-binutils```
```cargo install probe-rs --features cli --locked```

These other ones depend on the OS, but could be nice to install also:
- gdb (multiarch) (old chain)
- openocd         (old chain)
- qemu-system-arm (if you want to do some simulations)

For fedora:
```sudo dnf install gdb openocd qemu-system-arm```

For more info:
- https://docs.rust-embedded.org/book/intro/index.html
- https://rtic.rs/2/book/en/

### Running (new stack)
The new stack is posed to run with just a cargo run. In case of doubt or error,
take a look at the runner at the .cargo/config.toml and at the probe-rs docs at
https://probe.rs/docs/tools/probe-rs/
On the date of instalation the version of probe-rs is 0.22.0, so it is somewhat
expected to have some breaking changes in the future. If all goes bad, we can
always go back to the old stack. In that case take a look at older versions of
the template since the config and main/examples have been changed to work with
the new one.

### Running (old stack, not sure if will still work)

- Run ```openocd``` on a terminal window at the root folder of the project.
- Build the binary with ```cargo build```
- Run ```gdb -x openocd.gdb -q <path_to_bins>``` (gdb might have another
name on your installation, this was for fedora linux)
- Issue the continue command

> ```cargo build --example blink``` builds the example
> at target/thumbv7-none-eabihf/debug/examples/blink

Look at https://docs.rust-embedded.org/book/start/hardware.html for better
details on the running part if you are not sure what each thing is doing

