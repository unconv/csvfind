# csvfind

This repository started out as a simple CSV search tool written in C for a YouTube video. It was my first ever C program.

It has since turned into somewhat of a competition of different programming languages: Which one can create the fastest tool?

**Currently, the Rust v3 implementation is the winner**

| Implementation       | Mean execution time |
| -------------------- | ------------------- |
| csvfind.php          | 71.4 ms             |
| csvfind_v2.php       | 29.0 ms             |
| csvfind_v2.php (jit) | 29.4 ms             |
| c-csvfind-test-orig  | 41.6 ms             |
| c-csvfind-test       | 46.8 ms             |
| rust-csvfind-test    | 88.4 ms             |
| rust-csvfindv2-test  | 43.5 ms             |
| rust-csvfindv3-test  | 14.6 ms             |

## Usage

To compile, run the following command

```console
$ echo 'For C sources'
$ gcc csvfind.c -O -o csvfind
$ echo 'For Rust sources'
$ rustc -C opt-level=3 -C lto -C codegen-units=1 -o csvfind csvfind.rs
```

To use it, use the command

```console
$ ./csvfind INPUT_FILE SEARCH_TERM [HEADER_NAME]
```

For example, to search in `products.csv` for rows that have `sunglasses` in the `name` column, use

```console
$ ./csvfind products.csv sunglasses name
```

## Videos

How I made it:
https://www.youtube.com/watch?v=-45gWVRLb-Q

1st C/PHP/Rust comparison:
https://www.youtube.com/watch?v=XPkhjIGlbbU

2nd C/PHP/Rust comparison:
_Coming soon..._

## Background

The `csvfind.c` is the program I have written. I created the program using help from ChatGPT. I avoided asking too direct questions, like "how to parse CSV files in C", but instead I asked "how to read a file line by line" and "how to split a string by a comma" etc. in order to actually learn something from this experience.

There is also a `csvfind_chatgpt.c` file that was created by ChatGPT via a single prompt. I made this prompt after I finished creating my own version. It works similarly, but it is not case-insensitive.

## PHP version

I also made a PHP version of the program to test the performance of my C program compared to PHP. Surprisingly my original C program was slower than the PHP version. I then optimized the C code to make it faster than the PHP version. The main problem was using the `to_lowercase` function along with `strstr`. By switching to `strcasestr` the program became a lot faster.

Later, @fezfez wrote a new PHP version, that was faster than the C version.

## Rust version

As I was testing the performance between the PHP version and the C version, I also created a Rust version to see whether it would be faster than the PHP version. It was not. However, @alan910127 created a Rust version that was actually **faster** than the PHP _and_ the C versions. Since then @fezfez has written a faster PHP version, that is actually faster than the Rust version _and_ the C version.

## Join the contest

Feel free to send a pull request if you can make the Rust / C / PHP version any faster. You can also create the tool in another programming language!
