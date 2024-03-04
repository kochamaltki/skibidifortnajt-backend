#!/bin/bash

touch SECRET
./setup-db.sh
./build.sh
./target/debug/projekt-backend
