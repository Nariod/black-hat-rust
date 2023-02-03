# MD5/SHA-1 cracker

## Usage

```
# hash_cracker <wordlist.txt> <sha1_hash>
# for example
$ hash_cracker wordlist.txt 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53
# or
$ hash_cracker wordlist.txt 95401a269a87015f41c501aa2bfff8428713b848
```


## Run

```
$ cargo run -- wordlist.txt 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53
$ cargo run -- wordlist.txt fa6203854071c682b146dfe911ba88b2
```


## Build

```
$ cargo build
```