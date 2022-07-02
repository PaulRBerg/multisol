// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.7.0;

import "./a/A.sol";

contract B is A2 {
    function b() external pure returns (uint256) {
        return 1;
    }
}
