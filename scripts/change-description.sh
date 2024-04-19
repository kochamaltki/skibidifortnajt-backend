#!/bin/bash

path="$1/api/post/change/description"

curl --location --request POST "$path" \
--cookie "token=$3" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "new_description" : "'"$2"'"
}'
