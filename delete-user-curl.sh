#!/bin/bash

curl --location --request POST 'localhost:8000/api/post/delete-user' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
	"user_id": 0
}'
