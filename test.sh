#!/bin/bash

ip=$1

token_0=$(./login-curl.sh $ip root toor)
tok_0=${token_0:1:-1}
./create-post.sh $ip hello welcome yo $tok_0
echo
./create-post.sh $ip hi welcome ipex $tok_0
echo
./create-post.sh $ip hello response yo $tok_0
echo
token_1=$(./signup-curl.sh $ip dr 1234)
tok_1=${token_1:1:-1}
./create-post.sh $ip fu response ipex $tok_1
echo
./delete-user-curl.sh $ip $tok_1
echo
token_2=$(./signup-curl.sh $ip dr_ 1234)
tok_2=${token_2:1:-1}
./upgrade-curl.sh $ip 1 $tok_0
echo
./create-post.sh $ip xd yo ipex $tok_2
echo
./login-curl.sh $ip dr_ 123
echo
token_2=$(./login-curl.sh $ip dr_ 1234)
tok_2=${token_2:1:-1}
./signup-curl.sh $ip dr_ 1234
echo
./react-curl.sh $ip 0 0 $tok_2
echo
./react-curl.sh $ip 0 1 $tok_2
echo
./react-curl.sh $ip 0 0 $tok_0
echo
./react-curl.sh $ip 0 1 $tok_0
echo
./react-curl.sh $ip 1 1 $tok_1
echo
