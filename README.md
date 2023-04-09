# csvfind

This is a CSV search tool written in C. It filters a CSV based on given search term and optional header name.

I created this C program as a part of my YouTube video. It is the first ever C program I have written. You can watch the video here: https://www.youtube.com/watch?v=-45gWVRLb-Q

The `csvfind.c` is the program I have written. I created the program using help from ChatGPT. I avoided asking too direct questions, like "how to parse CSV files in C", but instead I asked "how to read a file line by line" and "how to split a string by a comma" etc. in order to actually learn something from this experience.

There is also a `csvfind_chatgpt.c` file that was created by ChatGPT via a single prompt. I made this prompt after I finished creating my own version. It works similarly, but it is not case-insensitive.

## Usage

To compile, run the following command

```console
$ gcc csvfind.c -o csvfind
```

To use it, use the command
```console
$ ./csvfind INPUT_FILE SEARCH_TERM [HEADER_NAME]
```

For example, to search in `products.csv` for rows that have `sunglasses` in the `name` column, use
```console
$ ./csvfind products.csv sunglasses name
```
