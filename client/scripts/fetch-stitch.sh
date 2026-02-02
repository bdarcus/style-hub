#!/bin/bash

# fetch-stitch.sh
# Usage: ./fetch-stitch.sh <url> <output_path>

URL="$1"
OUTPUT="$2"

if [ -z "$URL" ] || [ -z "$OUTPUT" ]; then
  echo "Usage: $0 <url> <output_path>"
  exit 1
fi

mkdir -p "$(dirname "$OUTPUT")"

echo "Fetching Stitch artifact from: $URL"
curl -L -o "$OUTPUT" "$URL"

if [ $? -eq 0 ]; then
  echo "Successfully saved to $OUTPUT"
else
  echo "Failed to fetch artifact"
  exit 1
fi
