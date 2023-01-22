# Hexdump

Mini hexdump written in Rust for exploring file contents.

## Usage

Execute this program by passing a file path:

```shell
cargo run my/file.txt
```

Here is the hexdump from this README file:

```shell
❯ cargo run README.md
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hexdump README.md`
[0x00000000] 23 20 48 65 78 64 75 6d 70 0a 0a 4d 69 6e 69 20
[0x00000010] 68 65 78 64 75 6d 70 20 77 72 69 74 74 65 6e 20
[0x00000020] 69 6e 20 52 75 73 74 20 66 6f 72 20 65 78 70 6c
[0x00000030] 6f 72 69 6e 67 20 66 69 6c 65 20 63 6f 6e 74 65
[0x00000040] 6e 74 73 2e 0a 0a 23 23 20 55 73 61 67 65 0a 0a
[0x00000050] 45 78 65 63 75 74 65 20 74 68 69 73 20 70 72 6f
[0x00000060] 67 72 61 6d 20 62 79 20 70 61 73 73 69 6e 67 20
[0x00000070] 61 20 66 69 6c 65 20 70 61 74 68 3a 0a 0a 60 60
[0x00000080] 60 73 68 65 6c 6c 0a 63 61 72 67 6f 20 72 75 6e
[0x00000090] 20 6d 79 2f 66 69 6c 65 2e 74 78 74 0a 60 60 60
[0x000000a0] 0a 0a 59 6f 75 20 73 68 6f 75 6c 64 20 73 65 65
[0x000000b0] 20 61 20 73 69 6d 69 6c 61 72 20 6f 75 74 70 75
[0x000000c0] 74 3a 0a 0a 60 60 60 73 68 65 6c 6c 0a 0a 60 60
```
