#!/bin/bash

path="$1/api/post/comment"

curl --location --request POST "$path" \
--cookie "token=$4" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
		"post_id": '$2',
		"body": "'"$3"'"
}'
