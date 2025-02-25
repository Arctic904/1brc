# 1BRC

[**The One Billion Row Challenge**](https://www.morling.dev/blog/one-billion-row-challenge/)

Compute simple math over 1 billion rows, as fast as possible, without dependencies.

Modified, because this is really not a job for Java IMO. Let's Rust it up!

## Generate the data file

There is a feature-gated binary which can create the appropriate measurements list, as follows:

```sh
time cargo run --release --features generator --bin generate 1000000000
```

## Run the challenge

```powershell
$ cargo build --release; time target/release/1brc > $null
   Compiling one-billion-rows v0.1.0 (1brc)
    Finished release [optimized] target(s) in 0.62s
```

## Example measuremetns

```
Tel Aviv;13.1
Tampa;3.7
Ouagadougou;48
Lake Tekapo;8.8
Palermo;-13.3
Louisville;6.8
Mandalay;30.3
Dhaka;10.4
Singapore;27.6
Birao;52.6
```

Generate copied from https://github.com/coriolinus/1brc/
