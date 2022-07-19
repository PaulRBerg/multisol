# Multisol [![Github Actions][gha-badge]][gha] [![Crates.io][crates-badge]][crates] [![License: MIT][license-badge]][license]

[gha]: https://github.com/paulrberg/multisol/actions
[gha-badge]: https://github.com/paulrberg/multisol/actions/workflows/ci.yml/badge.svg
[crates]: https://crates.io/crates/multisol
[crates-badge]: https://img.shields.io/crates/v/multisol.svg
[license]: https://opensource.org/licenses/MIT
[license-badge]: https://img.shields.io/badge/License-MIT-blue.svg

Multisol is a CLI application for verifying Solidity contracts on Etherscan.

You give it the path to your target contract and it generates a directory with that contract and all its
dependencies at the same relative path. Multisol refactors the source code to import dependencies from their new location.

- [x] Designed to be used with Etherscan's multi-part compiler type
- [x] Framework agnostic, so it is compatible with Hardhat, Truffle, or vanilla Solidity projects
- [x] Compatible with contracts imported from "node_modules"

## Installation

### Homebrew

This works only if you're a user of **macOS**.

```sh
$ brew tap paulrberg/multisol
$ brew install multisol
```

### Cargo

Cargo works across operating systems, but it requires you to have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your machine.

```sh
$ cargo install multisol
```

### Binaries

Archives of precompiled binaries for multisol are available for macOS, Linux and Windows in our [releases
page](https://github.com/paulrberg/multisol/releases).

Use this installation option if you are in doubt with regard to the others.

## Example

The listing below will generate a folder named "multisol-balancesheet" that can be used to verify the source code for
a deployed instance of the `BalanceSheet.sol` contract part of the [hifi](https://github.com/hifi-finance/hifi) monorepo.

```sh
git clone https://github.com/hifi-finance/hifi /tmp/example
cd /tmp/example
yarn install
yarn build
cd packages/protocol
multisol contracts/BalanceSheet.sol
```

After the folder is generated, you go to the contract's tab on Etherscan, click the "Verify & Publish" button and select
the "Solidity (Multi-Part files)" option in the dropdown. Then, you upload the folder, hit submit and voilà, your
contract is verified on Etherscan!

## Limitations

- Aliased specific symbols like `import {symbol1 as alias} from "filename"` are not supported
- Doesn't work when two or more contract files share the same name
- Doesn't work when the contract files do not have the "sol" extension
- Not compatible with Yarn's [Plug'n'Play](https://yarnpkg.com/features/pnp)

## Acknowledgements

This project has been inspired by [Solt](https://github.com/hjubb/solt). Multisol is different in that it uses the multi-part compiler type, while Solt uses [standard json][using-the-compiler] input format.

## License

[MIT](./LICENSE.md) © Paul Razvan Berg

<!-- Links -->

[using-the-compiler]: https://docs.soliditylang.org/en/v0.8.15/using-the-compiler.html
