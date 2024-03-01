#!/bin/bash
# u have to create user dr and database projekt-db through postgresql
psql -d projekt-db -U dr < setup.sql
psql -d projekt-db -U dr < fill-db.sql
./build.sh
./target/debug/projekt-backend
