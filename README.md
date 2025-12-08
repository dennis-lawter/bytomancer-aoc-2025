# bytomancer-aoc-2025
My solution repository for Advent of Code 2025.

## Spoiler Warning
Within the `src/solutions` folders,
you will see my code to solve Advent of Code 2025.

If you wish to remain spoiler free,
avoid these folders!

## Objective
I've decided on a few objectives this year for Advent of Code.
- As I have done all years, I will not use any AI tools.
- I still wish to solve each problem as fast as I can using Rust.
- I'll work entirely from Neovim with LazyVim this year.

## AOC Framework

This project began in November 2022,
as I worked on solving the AOC 2021 problems.
From my time tinkering with these problems,
I decided to add on a few features to ease development
(and for the simple fun of it).

### Features

1. I've implemented an input downloader which retrieves input files via the web.
    - A `.env` file is required with `SESSION=<Session ID from your cookie>`.
    - Files are downloaded to a `_cache/` folder created in the project root.
    - If an input file is already found locally, that file is loaded instead.
2. Final submissions are sent automatically to the form.
    - Using the same `.env` as above,
      executing the program with the `-s` or `--submit`
      option will send the result to the website's submission URL.
    - The resulting page is scanned and outputs a result to the command line.
3. Arguments dictate the solution to be run.
    - After discovering significant re-use between the days,
      I decided to package my code together in a single project.
    - Execution is performed with `cargo run -- dXsY`,
      representing day X solution Y.
4. Colorization is used heavily.
    - Tracking outputs and debugging is much simpler,
      thanks to the `colored` crate.
5. Automatic generation of solution files.
    - Usage: `cargo run -- d01s1 -g`
    - Executing the program with the `-g` or `--generate`
      option will perform some metaprogramming,
      generating new rust files.

### Personal Times

Day   -Part 1-   -Part 2-
  8   02:20:45   02:33:28
  7   00:23:13   00:56:33
  6   00:21:55   00:59:35
  5   00:13:49   01:01:07
  4   00:29:18   00:34:31
  3   00:18:35   00:37:24
  2   00:15:21   01:11:18
  1   00:13:31   00:28:18

