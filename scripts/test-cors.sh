#!/bin/bash

ip=$1

token_0=$(./scripts/login.sh $ip admin admin false)
tok_0=${token_0:1:-1}
token_1=$(./scripts/signup.sh $ip dr 1234 false)
tok_1=${token_1:1:-1}
./scripts/create-post.sh $ip hello welcome yo $tok_1
echo
