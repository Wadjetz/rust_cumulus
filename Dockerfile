FROM ubuntu:17.10
COPY target/x86_64-unknown-linux-musl/release/rust_cumulus /opt/cumulus/
COPY static /opt/cumulus/static/
