#!/bin/bash
psql -d projekt-db -U dr < setup.sql
psql -d projekt-db -U dr < fill-db.sql
./build.sh
./target/debug/projekt-backend
