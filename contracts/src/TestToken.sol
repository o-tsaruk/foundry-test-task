// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract TestToken is ERC20 {
    constructor(address account) ERC20("TestToken", "TT") {
        _mint(account, 100 * 10**18); 
    }
}