#!/bin/bash

path="$1/api/post/add-image-to-post"

curl --location --request POST "$path" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
		"token": "'"$4"'",
        "image_id": '$2',
        "post_id": '$3'
}'
