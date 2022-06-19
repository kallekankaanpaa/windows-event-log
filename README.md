# windows-event-log

Windows event logging library

I took inspiration from [winlog](https://crates.io/crates/winlog) with some key differences.
First of all I used [windows](https://crates.io/crates/windows) bindings instead of [winapi](https://crates.io/crates/winapi).
Secondly this library allows users to create custom event sources instead of just event sources under Application.
