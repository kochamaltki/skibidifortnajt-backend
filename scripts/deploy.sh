#!/bin/bash

mkdir -p ./media/images
mkdir -p ./media/profile_pictures
touch SECRET
sqlite3 projekt-db < setup.sql
sqlite3 projekt-db < secret.sql
./scripts/build.sh
./target/debug/projekt-backend
