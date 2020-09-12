https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0

https://github.com/creativcoder/rust-arduino-blink

How to run Rust on Arduino Uno - Our first blink
https://creativcoder.dev/rust-on-arduino-uno

cargo new rust-arduino-blink
rustup override set nightly

2020 版: Rust の Arduino 向けプログラムで L チカする方法
https://asukiaaa.blogspot.com/2020/08/rust-arduino-.html

avrdude -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-atmega328p/release/blink.elf:e

/dev/cu.usbmodem141401

avrdude -patmega328p -carduino -P/dev/cu.usbmodem141401 -b115200 -D -Uflash:w:target/avr-atmega328p/release/blink.elf:e
