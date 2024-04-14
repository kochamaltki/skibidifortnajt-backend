#!/bin/bash

mkdir -p ./media/profile-pictures
mkdir -p ./media/images
touch SECRET

sqlite3 projekt-db < setup.sql
sqlite3 projekt-db < secret.sql

./scripts/build-aws.sh
cargo run --release --verbose --jobs 1
