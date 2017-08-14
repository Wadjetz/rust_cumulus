#!/bin/bash

cd client && yarn build && cd ../ && ROCKET_ENV=prod cargo run --release
