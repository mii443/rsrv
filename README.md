# Rust bindings for RV
Safe bindings for the [RV](https://github.com/mnurzia/rv).

# Example
```rust
use rsrv::rv::*;

fn main() {
    let cpu = RV::new(0x10000, vec![0x02A88893, 0x00000073]);

    while RV::step(cpu) != RV_EECALL {}

    println!(
        "Environment call @ {:08x}: {}",
        RV::get_pc(cpu),
        RV::get_r(cpu)[17]
    );
}
```
