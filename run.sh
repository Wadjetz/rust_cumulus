#!/bin/bash

cd client && yarn build && ROCKET_ENV=prod cargo run --release
