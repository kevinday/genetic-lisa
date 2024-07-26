#!/bin/bash

# Create an empty array to hold the filenames
files=()

# Find and sort the files numerically, then reverse the order
for file in $(find lisa -name "*.svg" | sort -V | tac); do
    echo "Processing $file"
    files+=("$file")
done

# Convert the files into a GIF using magick
magick -background black -loop 0 -delay 3 "${files[@]}" lisa.gif

# Adjust the delay of the final GIF using magick
magick lisa.gif \( +clone -set delay 500 \) +swap +delete lisa.gif