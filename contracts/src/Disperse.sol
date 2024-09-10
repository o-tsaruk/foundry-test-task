// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./BaseContract.sol";

interface IERC20 {
    function transfer(address recipient, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
}

contract Disperse is BaseContract {
    /**
     * @notice Disperse ETH to multiple wallets
     * @dev Reverts with `TransferFailed` error when `call` fails
     */
    function disperseETH(TransferData[] memory transferList) external payable {
        for (uint8 i = 0; i < transferList.length; i++) {
            address currentWallet = transferList[i].wallet;
            (bool sent, ) = currentWallet.call{value: transferList[i].amount}("");
            if (!sent) {
                revert TransferFailed(currentWallet);
            }
        }
        emit TransferCompleted(msg.sender);
    }

    /**
     * @notice Disperse ERC-20 tokens to multiple wallets
     * @dev Reverts with `TransferFailed` error when `token.transfer` fails
     */
    function disperseERC20(address tokenAddress, address sender, TransferData[] memory transferList) external {
        IERC20 token = IERC20(tokenAddress);

        for (uint8 i = 0; i < transferList.length; i++) {
            address currentWallet = transferList[i].wallet;
            bool success = token.transferFrom(sender, currentWallet, transferList[i].amount);
            if (!success) {
                revert TransferFailed(currentWallet);
            }
        }
        emit TransferCompleted(msg.sender);
    }
}
