// SPDX-License-Identifier: MIT
pragma solidity ^0.7.0;

import "./B.sol";

contract A is B {
    function a() external pure returns (uint256) {
        return 1;
    }
}
