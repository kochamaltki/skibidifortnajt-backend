#!/bin/bash

path="$1/api/post/signup"

curl -s --location --request POST "$path" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
	"user_name": "'"$2"'",
	"passwd": "'"$3"'",
	"remember_password": '$4'
}'
