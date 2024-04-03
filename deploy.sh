#!/bin/bash

mkdir -p ./media/profile-pictures
mkdir -p ./media/images
touch SECRET
./setup-db.sh
./build.sh
./target/debug/projekt-backend
