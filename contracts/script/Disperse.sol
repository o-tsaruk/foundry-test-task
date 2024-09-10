// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Disperse} from "../src/Disperse.sol";

contract DisperseScript is Script {
    Disperse public disperse;

    function run() public {
        vm.startBroadcast();

        disperse = new Disperse();
        console.log("Disperse contract is deployed.");

        vm.stopBroadcast();
    }
}
