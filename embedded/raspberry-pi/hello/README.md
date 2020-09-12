参考：
https://pixelspark.nl/2020/cross-compiling-rust-programs-for-a-raspberry-pi-from-macos

brew install arm-linux-gnueabihf-binutils
rustup target add arm-unknown-linux-musleabi

cargo build --target=arm-unknown-linux-musleabi

Arduino Uno で Rust を実行する方法
https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0
