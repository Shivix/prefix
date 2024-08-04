# PREtty FIX
![CI Status](https://img.shields.io/github/actions/workflow/status/shivix/prefix/rust.yml?branch=master)
![Latest Release](https://img.shields.io/github/v/release/shivix/prefix)

A commandline based pretty printer for FIX messages.

Based on a FIX4.4 dictionary, but is usable with all versions.

<img alt="Prefix showcase" src="https://github.com/Shivix/prefix/tree/master/examples/prefix.gif" width="1200" />

## Usage
input can be passed in as an argument or piped in:
```bash
prefix "8=FIX4.4|1=test|55=EUR/USD|10=123|"
echo "8=FIX4.4|1=test|55=EUR/USD|10=123|" | prefix
```
outputs:
```
BeginString = FIX4.4
Account = test
Symbol = EUR/USD
CheckSum = 123
```

Currently can use ^ and | and SOH as delimiters.

Use `prefix --help` or `man prefix` for more details.

## Piping
Unix piping greatly increases the potential uses. For example:
Parsing a log file and aligning the values for easy scan reading.
```bash
# Pipe the file contents to prefix which parses and pipes them to awk, which prints them aligned.
cat example.txt | prefix -v | awk '{printf("%-20s %-30s\n", $1,$3)}'
```
outputs:
```
BeginString          FIX.4.4
Account              TEST
Symbol               EUR/USD
ExecType             PartialFill
```
Or summarising a log file that includes FIX messages.
```bash
# Pipe the file contents to prefix which then summarises the FIX messages by instrument
cat example.log | prefix --summary 55 --only-fix | sort | uniq --count
```
outputs:
```
4 NewOrderSingle EUR/USD
4 ExecutionReport EUR/USD
2 NewOrderSingle USD/KRW
2 ExecutionReport USD/KRW
```

## Installation
Can be installed using:
```
cargo install prefix
```
## Issues
Any bugs/ requests can be added to the [issues](https://github.com/Shivix/prefix/issues) page on the github repository.
