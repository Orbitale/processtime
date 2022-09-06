# Ptime

`ptime` is an executable that allows you to run a process and display its execution time.

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
