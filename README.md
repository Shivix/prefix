# PREtty FIX

A commandline based pretty printer for FIX messages.

## Usage
pipe other commands into or out of prefix to use.

example:
```
echo 8=4.4^1=test | prefix
```

outputs:
```
BeginString = 4.4
Account = test
```

Currently can use ^ and | and SOH as delimiters

Use `prefix --help` to see possible arguments to pass it.

Comes with a manpage but cargo will currently not install it automatically

## Installation
Can be installed using:
```
cargo install prefix
```
## Issues
Any bugs/ requests can be added to the [issues](https://github.com/Shivix/prefix/issues) page on the github repository.
