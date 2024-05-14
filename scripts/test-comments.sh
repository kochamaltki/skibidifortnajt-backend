#!/bin/bash

ip=$1

token_1=$(./scripts/signup.sh $ip dr 1234 false)
tok_1=$token_1
./scripts/create-post.sh $ip msg tag1 tag2 $tok_1
echo
./scripts/comment.sh $ip 0 response $tok_1
echo
./scripts/delete-user.sh $ip $tok_1
echo
