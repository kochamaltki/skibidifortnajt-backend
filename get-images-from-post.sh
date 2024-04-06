#!/bin/bash

path="$1/api/get/images/from-post/$2"

curl --location --request GET "$path"
