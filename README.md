## rstat

`rstat` is a shell command for summarizing statistics of Rust source files.

### Usage

```sh
rstat -m src .
```

### Output

```
summary stats {number of files: 7, loc: 187, comments: 8, blanks: 20}
```

### Project Structure

1. Create a project named `rstat`.
2. Create source files in the `src` folder: `main.rs`, `srcstats.rs`, `error.rs`. Keep the `Cargo.toml` file.
3. Custom error handling:
   - In `error.rs`, create a struct `StatsError` to represent custom error types.
   - Implement 4 traits for `StatsError`: `fmt::Display`, `From<&str>`, `From<io::Error>`, `From<std::num::TryFromIntError>`.
4. Define source statistics calculation logic:
   - In `srcstats.rs`, create a struct `SrcStats` to define source metrics to be calculated.
   - Define two functions:
     - `get_src_stats_for_file()`: Takes a filename as a parameter and calculates source metrics for that file.
     - `get_summary_src_stats()`: Takes a directory name as a parameter and calculates source metrics for all files in that directory.
5. Write the `main()` function to accept command-line arguments:
   - Define a struct `Opt` to specify command-line arguments and flags for the shell command.
   - Accept the source directory name from the command line and call the `get_summary_src_stats` method from the `srcstats` module.
   - Include the `structopt` module dependency in the `Cargo.toml` file.