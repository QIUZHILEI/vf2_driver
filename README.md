# vf2_driver
Visionfive 2 Uart and sdio(tf card) driver.

```rust
use vf2_driver::{log, println, sd::SdHost, serial};
serial::init_log(log::LevelFilter::Info).unwrap();
let sd = SdHost;
sd.init().unwrap();
let mut buf = [0u8;512];
let addr = some_addr;
sd.read_block(some_addr,&mut buf).unwrap();
println!("{buf:?}");
```