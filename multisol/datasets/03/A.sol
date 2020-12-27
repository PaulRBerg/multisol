// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;

import "@paulrberg/contracts/token/erc20/Erc20.sol";
import "@paulrberg/contracts/math/CarefulMath.sol";

contract Foo is Erc20 {
    constructor() Erc20("Foo Token", "FOO", 18) {}
}
