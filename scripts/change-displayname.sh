#!/bin/bash

path="$1/api/post/change/display-name"

curl --location --request POST "$path" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "new_display_name" : "'"$2"'",
        "token": "'"$3"'"
}'
