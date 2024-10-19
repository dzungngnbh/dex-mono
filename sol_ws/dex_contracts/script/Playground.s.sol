// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

import "../src/Clearinghouse.sol";
import "../src/Endpoint.sol";
import "../src/mocks/OBMock.sol";
import "../src/OffchainOrderbook.sol";

contract PlaygroundScript is Script {
    function setUp() public {}

    bytes32 constant EIP712_DOMAIN_TYPEHASH =
    keccak256(
        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
        );
    function run() public {
        vm.startBroadcast();

        vm.stopBroadcast();
    }

    function _constructSubAccount(address sender, bytes12 subaccount) internal returns (bytes32) {
        return bytes32(abi.encodePacked(bytes20(sender), subaccount));
    }

    // domain calculation from onchain and offchain is correct.
    function domainSeparator(address verifyingContract) private pure returns (bytes32) {
        return
        keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH,
                keccak256("DEX"),
                keccak256("0.1.0"),
                1,
                verifyingContract
                )
            );
    }
}
