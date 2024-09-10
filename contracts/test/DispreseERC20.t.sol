// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Disperse} from "../src/Disperse.sol";
import {BaseContract} from "../src/BaseContract.sol";
import {TestToken} from "../src/TestToken.sol";

contract DisperseERC20Test is Test {
    Disperse disperse;
    TestToken testToken;
    address mainAccount = address(0x1);
    address[] recipients = [address(0x2), address(0x3), address(0x4)];

    uint256 public constant DECIMALS = 10 ** 18;

    function setUp() public {
        disperse = new Disperse();
        testToken = new TestToken(address(mainAccount));
        vm.prank(mainAccount);
        testToken.approve(address(disperse), 100 * DECIMALS);
    }

    function testDisperseSuccess() public {
        assertEq(testToken.balanceOf(address(mainAccount)), 100 * DECIMALS, "MainAccount should have 100 tokens");

        Disperse.TransferData[] memory transferList = new Disperse.TransferData[](2);
        transferList[0] = BaseContract.TransferData({wallet: recipients[0], amount: 10 * DECIMALS});
        transferList[1] = BaseContract.TransferData({wallet: recipients[1], amount: 20 * DECIMALS});

        disperse.disperseERC20(address(testToken), mainAccount, transferList);

        assertEq(testToken.balanceOf(recipients[0]), 10 * DECIMALS, "Recipient 1 should have 10 tokens");
        assertEq(testToken.balanceOf(recipients[1]), 20 * DECIMALS, "Recipient 2 should have 20 tokens");
    }

    function testInsufficientBalance() public {
        Disperse.TransferData[] memory transferList = new Disperse.TransferData[](3);
        transferList[0] = BaseContract.TransferData({wallet: recipients[0], amount: 50 * DECIMALS});
        transferList[1] = BaseContract.TransferData({wallet: recipients[1], amount: 40 * DECIMALS});
        transferList[2] = BaseContract.TransferData({wallet: recipients[2], amount: 20 * DECIMALS});

        vm.expectRevert();
        disperse.disperseERC20(mainAccount, address(testToken), transferList);
    }
}