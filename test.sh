#!/bin/bash

token=$(./login-curl.sh localhost:8000 root toor)
tok=${token:1:-1}
echo "$tok"
./fill-db-curl.sh localhost:8000 hello welcome yo $tok
./fill-db-curl.sh localhost:8000 hi welcome ipex $tok 
./fill-db-curl.sh localhost:8000 hello response yo $tok
