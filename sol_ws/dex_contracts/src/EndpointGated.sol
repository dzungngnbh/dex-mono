// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import "./libraries/MathSD21x18.sol";

// Endpoint Dependency Injection
abstract contract EndpointGated is OwnableUpgradeable {
    address internal endpointAddress;

    function setEndpoint(address _endpoint) public onlyOwner {
        endpointAddress = _endpoint;
    }

    function getEndpoint() public view returns (address) {
        return endpointAddress;
    }

    modifier onlyEndpoint() {
        require(
            msg.sender == endpointAddress,
            "EndpointGated: caller is not the endpoint"
        );
        _;
    }
}