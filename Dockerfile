FROM alpine

ADD target/x86_64-unknown-linux-musl/release/vrc-invite-bot /usr/bin
ADD start.sh /

ENTRYPOINT "/start.sh"