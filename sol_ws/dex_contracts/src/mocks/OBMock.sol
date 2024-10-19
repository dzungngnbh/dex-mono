// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;

import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import "forge-std/console2.sol";

contract OBMock is EIP712Upgradeable, UUPSUpgradeable, OwnableUpgradeable{
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {_disableInitializers();}

    function initialize() initializer public {
        console2.log("OffchainOrderbook.initialize");
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();
        __EIP712_init("DEX", "0.1.0");
    }

    ///@dev required by the OZ UUPS module
    function _authorizeUpgrade(address) internal override onlyOwner {}

    function getDomain() public view returns (bytes32) {
        return _domainSeparatorV4();
    }
}
