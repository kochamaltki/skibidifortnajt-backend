#!/bin/bash

path="$1/api/post/upload/image"

curl --location --request POST "$path" \
--cookie "token=$3" \
--header 'Content-Type: multipart/form-data' \
--form "file=@$2"
