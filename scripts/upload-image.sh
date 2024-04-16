#!/bin/bash

path="$1/api/post/upload/image"

curl --location --request POST "$path" \
--header 'Content-Type: multipart/form-data' \
--header 'token: '$3'' \
--form "file=@$2"
