#!/bin/bash

path="$1/api/post/react"

curl --location --request POST "$path" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "post_id": '$2',
		"reaction_type": '$3',
        "token": "'"$4"'"
}'
