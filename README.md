## Build Commands

Apple Silicon: ```rustup target add aarch64-apple-darwin && cargo build --release --target aarch64-apple-darwin```

Apple Intel: ```rustup target add x86_64-apple-darwin && cargo build --release --target aarch64-apple-darwin```

Linux: ```rustup target add x86_64-unknown-linux-gnu && cargo build --release --target x86_64-unknown-linux-gnu```