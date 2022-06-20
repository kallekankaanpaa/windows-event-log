# windows-event-log

[log](https://crates.io/crates/log) adapter for Windows EventLog

I took inspiration from [winlog](https://crates.io/crates/winlog) with some key differences.

1. I used [windows](https://crates.io/crates/windows) bindings instead of [winapi](https://crates.io/crates/winapi).
2. This library allows users to create custom event sources instead of just event sources under Application.

## Future

One of the key features I would love to add is customizalbe message files, but I haven't figured out a way to do it yet.

## License 

Copyright 2022 Kalle Kankaanpää

Licensed under either [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

