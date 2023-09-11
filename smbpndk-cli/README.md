# smbpndk-cli

This is a CLI program to access [SmbPndk](https://smbpndk.com/).

## Installation

One can install this program in different ways. 

### With Cargo

```bash
cargo install smbpndk-cli
```

### Homebrew (MacOS/Linux)

```bash
brew tap smbpndk/tap
brew install cli
```

### With NPM

```
npm i -g @smbpndk/cli
```

## Update

Simply rerun the installation command.

## Uninstall

```bash

# With cargo
cargo uninstall smbpndk-cli

# With npm
npm uninstall -g @smbpndk/cli

# With Homebrew
brew untap smbpndk/tap 
brew uninstall smbpndk/tap/cli
```

## Usage:

```bash
smb --help
```

## Contribution

- Setup your Rust tooling.
- Clone the repo.
- Provide the environement variables in the .env.local.
- Run `cargo run`.

## Credits

This repo is inspired by [Sugar](https://github.com/metaplex-foundation/sugar).

This repo tries to follow [the 12 factor CLI app](https://medium.com/@jdxcode/12-factor-cli-apps-dd3c227a0e46) principles by Heroku team.

NPM support guide by [orhun.dev](https://blog.orhun.dev/packaging-rust-for-npm/).

## Licence

MIT.
