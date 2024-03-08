#!/bin/bash

token_0=$(./login-curl.sh localhost:8000 root toor)
tok_0=${token_0:1:-1}
./create-post.sh localhost:8000 hello welcome yo $tok_0
echo
./create-post.sh localhost:8000 hi welcome ipex $tok_0
echo
./create-post.sh localhost:8000 hello response yo $tok_0
echo
token_1=$(./signup-curl.sh localhost:8000 dr 1234)
tok_1=${token_1:1:-1}
./create-post.sh localhost:8000 fu response ipex $tok_1
echo
./delete-user-curl.sh localhost:8000 $tok_1
echo
token_2=$(./signup-curl.sh localhost:8000 dr_ 1234)
tok_2=${token_2:1:-1}
./upgrade-curl.sh localhost:8000 1 $tok_0
echo
./create-post.sh localhost:8000 xd yo ipex $tok_2
echo
./login-curl.sh localhost:8000 dr_ 123
echo
token_2=$(./login-curl.sh localhost:8000 dr_ 1234)
tok_2=${token_2:1:-1}
./signup-curl.sh localhost:8000 dr_ 1234
echo
./ban-curl.sh localhost:8000 1 $tok_2
echo
