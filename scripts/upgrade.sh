#!/bin/bash

path="$1/api/admin/post/upgrade-user"

curl --location --request POST "$path" \
--cookie "token=$3" \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
	"user_id": '$2'
}'
