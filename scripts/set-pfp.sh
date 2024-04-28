#!/bin/bash

path="$1/api/post/set-pfp"

curl --location --request POST "$path" \
--cookie "token=$4" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "image_id": '$2',
        "user_id": '$3'
}'
