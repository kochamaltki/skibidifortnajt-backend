#!/bin/bash

ip=$1

token_1=$(./scripts/login.sh $ip admin admin false)
tok_1=$token_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/react.sh $ip 0 $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/react.sh $ip 1 $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
echo
