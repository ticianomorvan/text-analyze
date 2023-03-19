This is a minimal utility for "analyzing" text files. It prints a report including total count of words, characters, whitespaces and so on.

```
Note: I'm not thinking of uploading it as a crate, as it's just a small program using the standard library.
```

## Instructions

1. Clone this repository with `git`

```sh
$ git clone https://github.com/ticianomorvan/text-analyze
```

2. Build the program with `cargo` (if you don't have it installed, you can get it at [Rust official page](https://www.rust-lang.org/tools/install))

```sh
cargo build --release
```

3. Run the program with `cargo`
```sh
cargo run --release
```

4. Input an absolute or relative path:

- Absolute path, e.g.: `/home/ticiano/Downloads/test.txt`
- Relative path, e.g.: `test.txt` or `../test.txt`

5. Get your file's report

```
This is the report for your file:
Alphabetic characters: 8821
Numeric characters: 0
Whitespaces: 1582
Total characters: 10612
Total words: 1569
```

**Note**: You can run the built executable after step **2**, using executable syntax `./text-analyze`