#!/bin/bash

curl --location --request POST 'localhost:8000/api/post/add-post' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
		"post_id": 0,
        "user_id": 1,
		"date": 0,
        "body": "hi"
}'
curl --location --request POST 'localhost:8000/api/post/add-post' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
		"post_id": 1,
        "user_id": 2,
		"date": 1,
        "body": "hi 1"
}'
