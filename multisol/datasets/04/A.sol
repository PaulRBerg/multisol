// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.0;

import { Erc20 } from "@paulrberg/contracts/token/erc20/Erc20.sol";
import { CarefulMath} from "@paulrberg/contracts/math/CarefulMath.sol";

contract Foo is Erc20 {
    constructor() Erc20("Foo Token", "FOO", 18) {}
}
