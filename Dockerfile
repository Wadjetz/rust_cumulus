FROM ubuntu:17.10

RUN apt-get update && apt-get install -y libssl-dev

COPY target/release/rust_cumulus /opt/cumulus/
COPY static /opt/cumulus/static/
