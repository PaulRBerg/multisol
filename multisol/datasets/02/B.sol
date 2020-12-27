// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;

import "./c/C.sol";

contract B is C {
    function b() external pure returns (uint256) {
        return 1;
    }
}
