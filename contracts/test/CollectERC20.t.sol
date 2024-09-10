// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Collect} from "../src/Collect.sol";
import {WithdrawalContract} from "../src/Collect.sol";
import {BaseContract} from "../src/BaseContract.sol";
import {TestToken} from "../src/TestToken.sol";

contract CollectERCTest is Test {
    Collect collect;
    TestToken testToken;
    address mainAccount = address(0x1);
    address[] senders = [address(0x2), address(0x3), address(0x4)];
    address receiver = address(0x5);

    function setUp() public {
        collect = new Collect();
        testToken = new TestToken(mainAccount);

        for (uint256 i = 0; i < senders.length; i++) {
            vm.prank(mainAccount);
            testToken.transfer(senders[i], 20);
            vm.prank(senders[i]);
            testToken.approve(address(collect), 10);
        }
    }

    function getAmountArray() private pure returns (uint256[] memory) {
        uint256[] memory amountArray = new uint256[](5);
        amountArray[0] = 1;
        amountArray[1] = 1;
        amountArray[2] = 1;
        return amountArray;
    }

    function testCollectERCSuccuess() public {
        assertEq(testToken.balanceOf(senders[0]), 20);
        assertEq(testToken.allowance(senders[0], address(collect)), 10);

        assertEq(testToken.balanceOf(receiver), 0);
        
        collect.collectERC20(address(testToken), receiver, senders, getAmountArray());

        assertEq(testToken.balanceOf(receiver), 3);
    }

    function getBigAmountsArray() private pure returns (uint256[] memory) {
        uint256[] memory amountArray = new uint256[](5);
        amountArray[0] = 10;
        amountArray[1] = 11;
        return amountArray;
    }

    function testInsufficientAllowance() public {
        vm.expectRevert(
            abi.encodeWithSignature(
                "ERC20InsufficientAllowance(address,uint256,uint256)",
                address(collect),
                10,
                11
            )
        );
        collect.collectERC20(address(testToken), receiver, senders, getBigAmountsArray());
    }
}
