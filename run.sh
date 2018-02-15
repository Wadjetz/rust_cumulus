#!/bin/bash

cd client && npm run build && cd ../ && ROCKET_ENV=prod cargo run --release
