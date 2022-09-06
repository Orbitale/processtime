# Process time

`processtime` is an executable that allows you to run a process and display its execution time.

## Install

### Using Cargo

You can run `cargo install processtime` if you have Rust and Cargo on your machine.

### Building from source

Run these commands:
```
git clone https://github.com/Orbitale/processtime
cd processtime
cargo build --release
```

This will create a `target/release/processtime` executable (or `processtime.exe` on Windows) that you can take and move anywhere you want.

## Usage

You can run `processtime` followed by any command, like this for instance:

```
$ processtime cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s

6s 460ms 994us 400ns
```

The last line will always display the time it took to run your command.

### Change the output format

By default, `processtime` displays a human-readable version of the execution time.

However, you might want to gather the information from a script or something and use it in other tools.

For that, you can use the `--format` option, which can take the following values:

* `full`: Human-readable (default format)
* `s`: Seconds (will output `0` for scripts that take less than 1 second to run)
* `ms`: Milliseconds
* `us` or `µs`: Microseconds 
* `ns`: Nanoseconds

**Note:** If you use this option, you should use the `--` separator to make sure `processtime` interprets your command properly, like this for example:

```
$ processtime --format=ms -- find . -iname "*.json"
```

This way, `processtime` interprets **everything** at the right of the `--` characters to be your command to execute.
