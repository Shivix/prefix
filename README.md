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

## Piping
Unix piping greatly increases the potential uses. For example:
Parsing a log file and aligning the values for easy scan reading.
```bash
# Creates the example log file.
echo "8=FIX.4.4|1=TEST^55=EUR/USD|150=1" > example.log
# Pipes the contents to prefix which parses and pipes them to awk, which prints them aligned.
cat example.log | prefix -v | awk '{printf("%-20s %-30s\n", $1,$3)}'
```
outputs:
```
BeginString          FIX.4.4
Account              TEST
Symbol               EUR/USD
ExecType             PartialFill
```

## Installation
Can be installed using:
```
cargo install prefix
```
## Issues
Any bugs/ requests can be added to the [issues](https://github.com/Shivix/prefix/issues) page on the github repository.
