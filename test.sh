#!/bin/bash

ip="localhost:8000"
token_0=$(./login-curl.sh $ip root toor)
tok_0=${token_0:1:-1}
./create-post.sh $ip hello welcome yo $tok_0
echo
./create-post.sh $ip hello welcome yo $tok_0
echo
./create-post.sh $ip hello welcome yo $tok_0
echo
sqlite3 projekt-db < test-posts.sql
./react-curl.sh $ip 0 $tok_0
echo
sqlite3 projekt-db < test-posts.sql
token_1=$(./signup-curl.sh $ip dr 1234)
tok_1=${token_1:1:-1}
./react-curl.sh $ip 0 $tok_1
echo
./react-curl.sh $ip 1 $tok_1
echo
./react-curl.sh $ip 2 $tok_1
echo
sqlite3 projekt-db < test-posts.sql
./ban-curl.sh $ip 1 $tok_0
echo
sqlite3 projekt-db < test-posts.sql
