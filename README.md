# Multisol

Multisol is a CLI application for verifying Solidity contracts on Etherscan.

You give it the path to your target contract and it generates a directory with that contract and all its
dependencies in the same directory. Multisol refactors the source code to import dependencies from their new location.

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

This works across operating systems, but it requires you to have [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your machine.

```sh
$ cargo install multisol
```

### Binaries

Archives of precompiled binaries for multisol are available for macOS, Linux and Windows in our [releases
page](https://github.com/paulrberg/multisol/releases).

Use this installation option if you are in doubt with regard to the others.

## Example

The code listing below will generate a folder named "multisol-fintroller" that can be used to verify the source code for
the `Fintroller.sol` contract on Etherscan.

```sh
git clone https://github.com/mainframehq/mainframe-lending-protocol.git /tmp/example
cd /tmp/example
yarn install
multisol contracts/Fintroller.sol
```

## Limitations

- Works only with [global-level
  imports](https://docs.soliditylang.org/en/v0.7.5/layout-of-source-files.html#syntax-and-semantics) like `import "./Foo.sol"`. Imports for specific symbols like `import {Foo} from "./Foo.sol"` aren't recognised yet.
- Doesn't work when two or more contract files share the same name
- Doesn't work when the contract files do not have the "sol" extension
- Not compatible with Yarn's [Plug'n'Play](https://yarnpkg.com/features/pnp)

## Acknowledgements

This project has been inspired by [Solt](https://github.com/hjubb/solt), developed by @hjubb. Multisol is different in
that it uses the multi-part compiler type, while Solt uses [standard json](https://docs.soliditylang.org/en/v0.8.0/using-the-compiler.html) input format.

## License

Software released under the [MIT License](./LICENSE.md).
