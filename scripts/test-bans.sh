#!/bin/bash

ip=$1

token_0=$(./scripts/login.sh $ip root toor)
tok_0=${token_0:1:-1}
token_1=$(./scripts/signup.sh $ip abc abc)
tok_1=${token_1:1:-1}
token_2=$(./scripts/signup.sh $ip def def)
tok_2=${token_2:1:-1}
./scripts/create-post.sh $ip abc a b $tok_1
./scripts/create-post.sh $ip def d e $tok_2
./scripts/ban.sh $ip 1 1000 "za bycie sprzedajna kurwa" $tok_0
echo
./scripts/login.sh $ip abc abc
echo
./scripts/unban.sh $ip 1 $tok_0
./scripts/login.sh $ip abc abc
echo
