// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../lib/openzeppelin-contracts/contracts/access/Ownable.sol";
import "./BaseContract.sol";

interface IERC20 {
    function transferFrom(
        address sender,
        address recipient,
        uint256 amount
    ) external returns (bool);
}

contract WithdrawalContract {
    receive() external payable {}

    function withdraw(address to, uint256 _amount) external {
        require(address(this).balance >= _amount, "Insufficient balance");
        (bool success, ) = to.call{value: _amount}("");
        require(success, "Transfer failed");
    }
}

contract Collect is BaseContract, Ownable {
    WithdrawalContract[] public withdrawalContracts;

    uint8 public constant WITHDRAWAL_CONTRACTS_NUMBER = 5;

    constructor() Ownable(msg.sender) {}

    function createWithrawalContracts() external onlyOwner() {
        for (uint256 i = 0; i < WITHDRAWAL_CONTRACTS_NUMBER; i++) {
            WithdrawalContract newContract = new WithdrawalContract();
            withdrawalContracts.push(newContract);
        }
    }

    function getWithdrawalContracts() public view returns (WithdrawalContract[] memory) {
        return withdrawalContracts;
    }

    function collectETH(uint256[] calldata amounts) external onlyOwner {
        for (uint256 i = 0; i < amounts.length; i++) {
            withdrawalContracts[i].withdraw(owner(), amounts[i]);
        }
    }   

    /**
     * @notice Collect ERC-20 tokens to `receiver` wallet
     * @dev Reverts with `TransferFailed` error when `erc20.transferFrom` fails
     */
    function collectERC20(
        address token,
        address receiver,
        address[] calldata senders,
        uint256[] calldata amounts
    ) external onlyOwner {
        IERC20 erc20 = IERC20(token);

        for (uint256 i = 0; i < senders.length; i++) {
            bool success = erc20.transferFrom(senders[i], receiver, amounts[i]);
            if (!success) {
                revert TransferFailed(senders[i]);
            }
        }

        emit TransferCompleted(msg.sender);
    }
}
