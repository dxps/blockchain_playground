## Solana Escrow sample

This sample is based on [this nice](https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/) article.

### Prereqs

As prerequisites (aka environment setup), we need to have [Rust](https://rustup.rs/) and [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool) installed.

### Building

To build and test for program compiled natively, use:
```
$ cargo build
$ cargo test
```

To build and test the program compiled for BPF, use:
```
$ cargo build-bpf
$ cargo test-bpf
```


