#!/bin/bash

path="$1/api/post/add-post"

curl --location --request POST "$path" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "user_id": '$2',
        "body": "'"$3"'",
        "token": "'"$4"'"
}'
