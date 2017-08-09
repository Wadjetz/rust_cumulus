#!/bin/bash

cargo clean

docker pull wadjetz/rust-build

docker run -it -v `pwd`:/opt/project wadjetz/rust-build:latest /root/.cargo/bin/cargo build --release

docker build --no-cache -t wadjetz/cumulus:0.0.16 .
