// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Collect} from "../src/Collect.sol";
import {WithdrawalContract} from "../src/Collect.sol";
import {BaseContract} from "../src/BaseContract.sol";

contract CollectETHTest is Test {
    Collect collect;
    uint256 constant DECIMALS = 10 ** 18;

    function setUp() public {
        vm.deal(address(0x1), 15 ether);
        vm.prank(address(0x1));
        collect = new Collect();
    }

    function testCollectHaveOwner() public view {
        assertEq(address(0x1), collect.owner());
    }

    function testCollectGeneratesContracts() public {
        assertEq(collect.getWithdrawalContracts().length, 0);

        vm.prank(address(0x1));
        collect.createWithrawalContracts();

        assertEq(collect.getWithdrawalContracts().length, 5);
    }

    function testCollectFundsContracts() public {
        vm.prank(address(0x1));
        collect.createWithrawalContracts();

        WithdrawalContract[] memory generatedContracts = collect
            .getWithdrawalContracts();

        assertEq(address(0x1).balance, 15 * DECIMALS);

        for (uint256 i = 0; i < 5; i++) {
            vm.prank(address(0x1));
            payable(address(generatedContracts[i])).transfer(3 ether);
        }

        assertEq(address(generatedContracts[0]).balance, 3 * DECIMALS);
    }

    function testCollectSuccessfullWithdraw() public {
        vm.prank(address(0x1));
        collect.createWithrawalContracts();

        WithdrawalContract[] memory generatedContracts = collect
            .getWithdrawalContracts();

        for (uint256 i = 0; i < 5; i++) {
            vm.prank(address(0x1));
            payable(address(generatedContracts[i])).transfer(3 ether);
        }

        assertEq(address(0x1).balance, 0);

        assertEq(generatedContracts.length, 5);
        assertEq(getAmountArray().length, 5);

        vm.prank(address(0x1));
        collect.collectETH(getAmountArray());

        assertEq(address(0x1).balance, 7 * DECIMALS);
    }

    function getAmountArray() private pure returns (uint256[] memory) {
        uint256[] memory amountArray = new uint256[](5);
        amountArray[0] = 1 * DECIMALS;
        amountArray[1] = 1 * DECIMALS;
        amountArray[2] = 1 * DECIMALS;
        amountArray[3] = 1 * DECIMALS;
        amountArray[4] = 3 * DECIMALS;
        return amountArray;
    }
}
