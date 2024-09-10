// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Collect} from "../src/Collect.sol";

contract DisperseScript is Script {
    Collect public collect;

    function run() public {
        vm.startBroadcast();

        collect = new Collect();
        console.log("Collect contract is deployed.");

        vm.stopBroadcast();
    }
}
