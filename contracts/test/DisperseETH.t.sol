// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Disperse} from "../src/Disperse.sol";
import {BaseContract} from "../src/BaseContract.sol";

contract RevertingContract {
    receive() external payable {
        revert("RevertingContract: ETH transfer rejected");
    }
}

contract DisperseETHTest is Test {
    Disperse disperse;
    RevertingContract revertingContract;
    address payable public recipient1;
    address payable public recipient2;

    function setUp() public {
        disperse = new Disperse();
        revertingContract = new RevertingContract();
        recipient1 = payable(address(0x1));
        recipient2 = payable(address(0x2));
    }

    receive() external payable {}

    function testDisperseSuccess() public {
        Disperse.TransferData[] memory transferList = new Disperse.TransferData[](2);
        transferList[0] = BaseContract.TransferData({ wallet: recipient1, amount: 1 ether });
        transferList[1] = BaseContract.TransferData({ wallet: recipient2, amount: 2 ether });

        vm.expectEmit(true, false, false, false);
        emit BaseContract.TransferCompleted(address(this));

        disperse.disperseETH{value: 3 ether}(transferList);

        assertEq(address(recipient1).balance, 1 ether);
        assertEq(address(recipient2).balance, 2 ether);
    }

    function testRevertingContract() public {
        Disperse.TransferData[] memory transferList = new Disperse.TransferData[](1);
        transferList[0] = BaseContract.TransferData({ wallet: address(revertingContract), amount: 1 ether });

        vm.expectRevert(abi.encodeWithSignature("TransferFailed(address)", address(revertingContract)));
        disperse.disperseETH{value: 1 ether}(transferList);
    }
}
