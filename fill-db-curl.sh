#!/bin/bash

curl --location --request POST 'localhost:8000/api/post/add-post' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "user_id": 0,
        "body": "hi"
}'
curl --location --request POST 'localhost:8000/api/post/add-post' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "user_id": 1,
        "body": "hi dr"
}'
