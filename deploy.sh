#!/bin/bash
sqlite3 projekt-db < setup.sql
./build.sh
./target/debug/projekt-backend
