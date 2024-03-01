#!/bin/bash
# u have to create user dr and database projekt-db through postgresql
sqlite3 projekt-db < setup.sql
sqlite3 projekt-db < fill-db.sql
./build.sh
./target/debug/projekt-backend
