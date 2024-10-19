// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "../src/Clearinghouse.sol";
import "../src/Endpoint.sol";

contract UpgradeDexScript is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        address endpointProxyAddress = vm.envAddress("ENDPOINT_PROXY_ADDRESS");
        console2.log("Endpoint deployed at address: %s", endpointProxyAddress);

        require(endpointProxyAddress.code.length != 0, "endpointProxyAddress contains no code.");

        // test running some functions before upgrading
        Endpoint endpoint = Endpoint(endpointProxyAddress);

        address sequencerAddress = vm.envAddress("SEQUENCER_ADDRESS");
        address clearinghouseAddress = vm.envAddress("CLEARINGHOUSE_ADDRESS");

        // TODO: storage migration
        address newEndpointImplementation = address(new Endpoint());
        endpoint.upgradeToAndCall(newEndpointImplementation, "");

        vm.stopBroadcast();
    }
}
