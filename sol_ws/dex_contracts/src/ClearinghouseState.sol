// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "./interfaces/engine/IProductEngine.sol";

abstract contract ClearinghouseState {
    address internal quote;
    IProductEngine.EngineType[] internal supportedEngines;
    mapping(IProductEngine.EngineType => address) internal engineAddressByType;
    mapping(uint32 => IProductEngine) internal productIdToEngine;

    // number of products across engine
    uint32 internal numProducts;
}
