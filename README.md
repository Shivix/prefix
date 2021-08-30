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

Currently can use ^ and | as delimiters. SOH will not currently work.

cat -v can be used to convert SOH into ^A which will be parsed fine.

## Installation
Can be installed using:
```
cargo install prefix
```

