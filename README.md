# SRM (Selection Remove)

`SRM` tool allows you to list, select, and delete files and folders in the current directory, with optional filtering using regular expressions.

## Usage
Run the tool with the following syntax:

```sh
cargo run [<regex_pattern>] [OPTIONS]
```

## Examples
List all files and folders (without any filtering):

```sh
cargo run
```

Filter files with a regex pattern:

```sh
cargo run ".*\\.rs"
```

Delete files with regex filter and skip errors:

```sh
cargo run "*.toml" -s
```

### Options
* `--help`: Displays usage information.
* `--version`: Shows the version of the tool.
* `-s`: Skips errors that occur during file deletion.
