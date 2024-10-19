// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {Endpoint} from "../src/Endpoint.sol";
import "./TestHelper.sol";

contract EndpointTest is Test, TestHelper {
    Endpoint public endpoint;

    function setUp() public {
        vm.startBroadcast(FIRST_ACC);
        endpoint = new Endpoint();
        vm.stopBroadcast();
    }

    function testDepositCollateral() public payable {
        vm.startBroadcast(SECOND_ACC);
//        endpoint.doSomething();
        vm.stopBroadcast();
    }
}