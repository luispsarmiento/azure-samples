# azure-templates
A collection of templates that implement various Azure cloud services.

## Send Email function
### Compile to run localy
```bash
cargo build --release
cp target/release/send-email ./send-email-result
func start
```
### Compile to deploy
Compile the send-email function to Linux/x64. A binary named send-email is created. Copy it to the function app root.
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/send-email ./send-email-af
```