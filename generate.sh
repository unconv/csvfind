#!/usr/bin/env bash

# Check if products.csv exists
if [ ! -f products.csv ]; then
    echo "File products.csv not found!"
    exit 1
fi

# Empty the output file if it already exists, or create a new one
> output.csv

# Repeat the contents of products.csv 10000 times and append to output.csv
cat products.csv >> output.csv
for i in {1..9999}; do
    sed '1d' products.csv >> output.csv
done

# Modify the last line
LAST_LINE=$(sed -n '${0,/ThinkPad/s//WinkPad/;p}' output.csv)

# Delete the last line
sed -i '$d' output.csv

# Write the modified line to the end of the file
echo $LAST_LINE >> output.csv

echo "Task completed: products.csv content repeated 10000 times in output.csv and modified the last line."
