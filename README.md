# sprectre-rust-poc
Spectre PoC in Rust

The PoC is a basic implementation of send->receive through a cache side-channel.
It may be a full SpectreV2 implementation in the future - if I find a good BTI gadget to abuse. The current limitation is that no indirect call has the _secret_ byte in a register prior to execution.

## Prerequisites
```
$rustc -V
rustc 1.46.0-nightly (feb3536eb 2020-06-09)
```

## Resources
[Rust Inline ASM RFC](https://github.com/Amanieu/rfcs/blob/inline-asm/text/0000-inline-asm.md)  
[Rust Inline ASM blog post](https://blog.rust-lang.org/inside-rust/2020/06/08/new-inline-asm.html)
