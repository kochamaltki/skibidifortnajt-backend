#!/bin/bash

curl --location --request POST 'localhost:8000/api/post/login' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
	"user_name": "dr",
	"passwd": "1234"
}'
