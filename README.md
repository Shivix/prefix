# PREtty FIX
A commandline based pretty printer for FIX messages.

Based on a FIX4.4 dictionary, but is usable with most versions.

## Usage
Mainly developed with unix piping in mind:
```bash
echo 8=4.4^1=test | prefix
```
But can also provide a message as an argument:
```bash
prefix 8=4.4^1=test
```

outputs:
```
BeginString = 4.4
Account = test
```

Currently can use ^ and | and SOH as delimiters.

Use `prefix --help` or `man prefix` for more details.

## Installation
Can be installed using:
```
cargo install prefix
```
## Issues
Any bugs/ requests can be added to the [issues](https://github.com/Shivix/prefix/issues) page on the github repository.
