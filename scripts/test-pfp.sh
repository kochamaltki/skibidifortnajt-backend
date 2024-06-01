#!/bin/bash

ip=$1

tok_0=$(./scripts/login.sh $ip admin admin false)
tok_1=$(./scripts/signup.sh $ip dr 1234 false)
echo $tok_0
echo $tok_1
./scripts/create-post.sh $ip hello welcome yo $tok_1
./scripts/upload-image.sh $ip "media/profile_pictures/default.png" $tok_0
./scripts/set-pfp.sh $ip 0 0 $tok_0
./scripts/remove-pfp.sh $ip 0 $tok_0
