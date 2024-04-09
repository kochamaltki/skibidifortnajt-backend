#!/bin/bash

ip="localhost:8000"
token_0=$(./login.sh $ip root toor)
tok_0=${token_0:1:-1}
token_1=$(./signup.sh $ip abc abc)
tok_1=${token_1:1:-1}
./ban.sh $ip 1 100 "za bycie sprzedajna kurwa" $tok_1
echo
./login.sh $ip abc abc
