// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {TestToken} from "../src/TestToken.sol";

contract TestTokenScript is Script {
    TestToken public token;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address receiver = vm.addr(deployerPrivateKey);
        vm.startBroadcast(deployerPrivateKey);

        token = new TestToken(receiver);
        console.log("Contract instance address:", address(token));

        vm.stopBroadcast();
    }
}
