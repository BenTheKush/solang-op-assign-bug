// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >0.7.0;

contract Issue1521 {
    uint256 private _totalSupply;

    function updateStorage() public {
        _totalSupply += 1;
        _totalSupply *= 1;
        _totalSupply /= 1;
        _totalSupply -= 1;
        _totalSupply &= 1;
        _totalSupply |= 1;
    }

    function updateVariable() public pure {
        uint256 x;
        x += 1;
        x *= 1;
        x /= 1;
        x -= 1;
        x &= 1;
        x |= 1;
    }
}
