#!/bin/bash

curl --location --request POST 'localhost:8000/api/post' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "user_id": 1,
        "body": "hi"
}'
curl --location --request POST 'localhost:8000/api/post' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "user_id": 1,
        "body": "f u"
}'
curl --location --request POST 'localhost:8000/api/post' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "user_id": 2,
        "body": "why?"
}'
curl --location --request POST 'localhost:8000/api/post' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "user_id": 3,
        "body": "why not"
}'
curl --location --request POST 'localhost:8000/api/post' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "user_id": 4,
        "body": "sure"
}'
