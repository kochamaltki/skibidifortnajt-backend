#!/bin/bash

mkdir -p ./media/profile-pictures
mkdir -p ./media/images
touch SECRET
./setup-db.sh
./build-aws.sh
cargo run --release --verbose --jobs 1
