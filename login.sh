#!/bin/bash

path="$1/api/post/login"

curl -s --location --request POST "$path" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
	"user_name": "'"$2"'",
	"passwd": "'"$3"'"
}'
