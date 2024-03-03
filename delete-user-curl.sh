#!/bin/bash

path="$1/api/post/delete-user"

curl --location --request POST "$path" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "user_id": '$2',
}'