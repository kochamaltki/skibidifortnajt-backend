#!/bin/bash

path="$1/api/post/delete-user"

curl --location --request POST "$path" \
--cookie "token=$2" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
}'
