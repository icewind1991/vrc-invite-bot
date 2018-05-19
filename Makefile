all: target/x86_64-unknown-linux-musl/release/vrc-invite-bot

target/x86_64-unknown-linux-musl/release/vrc-invite-bot: Cargo.toml src/main.rs
	docker run --rm -it -v "$(CURDIR):/home/rust/src" ekidd/rust-musl-builder cargo build --release

docker: target/x86_64-unknown-linux-musl/release/vrc-invite-bot Dockerfile
	docker build -t icewind1991/vrc-invite-bot .