#!/bin/bash

path="$1/api/admin/post/ban-user"

curl --location --request POST "$path" \
--cookie "token=$5" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
	"user_id": '$2',
	"ban_length": '$3',
	"ban_message": "'"$4"'"
}'
