#!/bin/bash
sqlite3 projekt-db < setup.sql
sqlite3 projekt-db < fill-db.sql
./build.sh
./target/debug/projekt-backend
