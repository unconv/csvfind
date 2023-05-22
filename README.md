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

## PHP version

I also made a PHP version of the program to test the performance of my C program compared to PHP. Surprisingly my original C program was slower than the PHP version. I then optimized the C code to make it faster than the PHP version. The main problem was using the `to_lowercase` function along with `strstr`. By switching to `strcasestr` the program became a lot faster.

## Rust version

Thanks to @alan910127, there is now a Rust version of the program that is in fact **faster** than the PHP *and* the C versions! You can find it in the `csvfind_v2.rs` file.

If you can make a faster version in C (or PHP), feel free to make a pull request!

<s>As I was testing the performance between the PHP version and the C version, I also created a Rust version to see whether it would be faster than the PHP version. It is not. I have not yet found a solution to make the Rust version faster than the PHP version. Most of the slowness seems to come from using `to_lowercase` with `contains` in the loop. I have not yet found a performant method to search for a string in another string case-insensitively in Rust.

Feel free to send a pull request if you can make the Rust / C / PHP version any faster.</s>
