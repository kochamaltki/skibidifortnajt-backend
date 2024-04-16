#!/bin/bash

path="$1/api/post/add-post"

curl --location --request POST "$path" \
--cookie "token=$5" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "body": "'"$2"'",
		"tags": [
			"'"$3"'",
			"'"$4"'"
		]
}'
