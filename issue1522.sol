// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

library Issue1522 {
    function i1522(int256 x) internal pure {
        require(x >= type(int256).min, "underflow");
    }
}
