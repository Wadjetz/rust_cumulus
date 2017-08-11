#!/bin/bash

cargo clean

cross build --release --target x86_64-unknown-linux-musl

docker build --no-cache -t wadjetz/cumulus:0.0.20 .
