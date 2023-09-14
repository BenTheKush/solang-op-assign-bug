// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

library Lib {
    using {f} for int256;

    function f(int256 a) internal pure returns (int256) {
        return a;
    }
}
