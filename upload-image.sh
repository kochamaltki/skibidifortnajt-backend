#!/bin/bash

path="$1/api/post/upload/image"

curl --location --request POST "$path" \
--header 'Content-Type: multipart/form-data' \
--form "file=@$2"
