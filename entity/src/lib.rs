[package]
name = "entity"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
arel = { workspace = true, features = ["runtime-tokio-native-tls", "postgres"] }