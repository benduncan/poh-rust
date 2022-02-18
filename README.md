# Proof of History - Concepts in Rust

```
 ________  ________  ___  ___         
|\   __  \|\   __  \|\  \|\  \        
\ \  \|\  \ \  \|\  \ \  \\\  \       
 \ \   ____\ \  \\\  \ \   __  \      
  \ \  \___|\ \  \\\  \ \  \ \  \ ___ 
   \ \__\    \ \_______\ \__\ \__\\__\
    \|__|     \|_______|\|__|\|__\|__| rust!
                           
```

## Objective

Provide source-code concepts from the [Solana whitepaper](https://github.com/solana-labs/whitepaper/blob/master/solana-whitepaper-en.pdf) in Rust to understand proof-of-history and the mechanics behind Solana further.

Used as an exercise to understand Proof of History further by writing a simple implementation as described in the whitepaper. 

## Results

```
$ cargo build --release ; time ./target/release/rust-ug-crypto --name test

0 => E0C54B3441E3AF7683D6094C03B806534D0A0AC3EE70CF775260BDAEAA6B8080
100000 => A6A9D5C3BA9BF2A27BC07CAE0A51D7A65F861CB61B7F589AE08E4DD5502E565B
200000 => 820B54599E3687795A246FDBA4F91C033AF8128D1679E6ED2DE5E8FCFF32D4CD
...
10000000 hashes took - duration 4.359454666s

./target/release/rust-ug-crypto --name test  4.34s user 0.03s system 89% cpu 4.862 total
```
