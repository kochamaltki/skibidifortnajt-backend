#!/bin/bash

ip=$1

token_1=$(./scripts/signup.sh $ip dr 1234 false)
tok_1=$token_1
echo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
echo
