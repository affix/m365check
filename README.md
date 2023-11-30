# Microsoft365 User Verification CLI

## Overview
m365check is a command-line interface (CLI) application designed to verify the validity of Microsoft365 user accounts without making login requests. This tool provides a fast and efficient way to check if Microsoft365 user accounts are active, without the need to authenticate each user.

## Features
- Single Email Check: Quickly check the validity of a single Microsoft365 user account by providing an email address.
- Bulk Email Check: Verify multiple accounts simultaneously by providing a file containing a list of email addresses.
- No Login Requests: Safely verify user accounts without triggering login requests or authentication processes.
- Easy-to-Use: A simple CLI interface makes it accessible for users of all technical levels.


# Installation

```
cargo install m365check

```

# Usage
The basic usage of m365check is outlined below:

```
Usage: m365check [OPTIONS]

Options:
  -e, --email <EMAIL>  E-mail address of a single user to check if no option is provided, the program will read from stdin [default: ]
  -f, --file <FILE>    File containing a list of e-mail addresses to check One e-mail address per line [default: ]
  -h, --help           Print help
  -V, --version        Print version
```

## Example Usage

To check a single E-mail address

```
m365check -e user@mail.com
```

To check a list of E-mail addresses

```
m365check -f /path/to/file.txt
```

To check an e-mail from stdin

```
echo "user@mail.com" | m365check
```

## License
[MIT](https://choosealicense.com/licenses/mit/)

