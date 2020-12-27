# Multisol

Multisol is a CLI application for verifying Solidity contracts on Etherscan.

You give it the path to your target contract and it generates a directory with that contract and all its
dependencies in the same directory. Multisol refactors the source code to import dependencies from their new location.

- [x] Designed to be used with Etherscan's multi-part compiler type
- [x] Framework agnostic, so it is compatible with Hardhat, Truffle, or vanilla Solidity projects
- [x] Works with contracts imported from "node_modules"

## Installation

For Mac:

```sh
$ brew install multisol
```

For Linux:

```sh
$ todo
```

## Example

Lorem ipsum.

## Limitations

- Works only with [global-level
  imports](https://docs.soliditylang.org/en/v0.7.5/layout-of-source-files.html#syntax-and-semantics) like `import "./Foo.sol"`. Imports for specific symbols like `import {Foo} from "./Foo.sol"` aren't recognised yet.
- Doesn't work when two or more contract files share the same name
- Doesn't work when the contract files do not have the "sol" extension
- Not compatible with Yarn's [Plug'n'Play](https://yarnpkg.com/features/pnp)

## Acknowledgements

This project has been inspired by [Solt](https://github.com/hjubb/solt), developed by @hjubb. Besides being written in
different programming languages, Multisol differs from Solt in that it uses the multi-part compiler type, while
the latter relies on the [standard json](https://docs.soliditylang.org/en/v0.8.0/using-the-compiler.html) input format.

## License

Software released under the [MIT License](./LICENSE.md).
