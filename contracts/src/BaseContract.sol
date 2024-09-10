// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract BaseContract {
    struct TransferData {
        address wallet;
        uint256 amount;
    }

    event TransferCompleted(address indexed sender);
    error TransferFailed(address to);
}
