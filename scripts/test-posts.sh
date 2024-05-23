#!/bin/bash

ip=$1

token_1=$(./scripts/login.sh $ip admin admin false)
tok_1=$token_1
tok_2=$(./scripts/signup.sh $ip t1 admin false)
tok_3=$(./scripts/signup.sh $ip t2 admin false)
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/react.sh $ip 0 $tok_1
./scripts/react.sh $ip 0 $tok_2
./scripts/react.sh $ip 0 $tok_3
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/react.sh $ip 1 $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/react.sh $ip 4 $tok_3
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/react.sh $ip 8 $tok_2
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
