# Gandi Dynamic DNS

`gandi-dyn-dns` is a program that can periodically refresh DNS records,
  if you have a dynamic IP. It supports both IPv4 and IPv6.

## Installation

`cargo install --release`

## Usage

If running as a user, create a config file (see `gandi-dyn-dns.example.toml`)

`~/.config/gandi-dyn-dns/gandi-dyn-dns.toml` (Linux)

`~/Library/Application Support/gandi-dyn-dns/gandi-dyn-dns.toml` (OSX)

`%APPDATA%\gandi-dyn-dns\gandi-dyn-dns.toml` (Windows)

## Contributing

Pull requests are welcome. For major changes, please open an
issue first to discuss what you would like to change.

## License

[GPLv3](https://choosealicense.com/licenses/gpl-3.0/)
