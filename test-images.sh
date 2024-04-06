#!/bin/bash

ip="localhost:8000"
token_0=$(./login.sh $ip root toor)
tok_0=${token_0:1:-1}
token_1=$(./signup.sh $ip dr 1234)
tok_1=${token_1:1:-1}
./create-post.sh $ip hello welcome yo $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png
echo
./add-image-to-post.sh $ip 0 0 $tok_1
echo
./get-images-from-post.sh $ip 0
echo
