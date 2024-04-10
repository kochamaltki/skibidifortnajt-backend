#!/bin/bash

ip=$1

token_0=$(./login.sh $ip root toor)
tok_0=${token_0:1:-1}
token_1=$(./signup.sh $ip dr 1234)
tok_1=${token_1:1:-1}
./create-post.sh $ip hello welcome yo $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_0
echo
sleep 6
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
./upload-image.sh $ip ./media/profile_pictures/default.png $tok_1
echo
