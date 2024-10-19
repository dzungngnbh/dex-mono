// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "../libraries/RiskHelper.sol";
import "./IEndpoint.sol";
import "./engine/IProductEngine.sol";
import "./engine/ISpotEngine.sol";

interface IClearinghouse {
    function initialize(address _endpoint, address _quote) external;

    function addEngine(address _engine, IProductEngine.EngineType _engineType) external;
    function getEngineByType(IProductEngine.EngineType engineType) external view returns (address);
    function getHealth(bytes32 subaccount, IEndpoint.HealthType healthType) external view returns (int128 health);
    function getSupportedEngines() external view returns (IProductEngine.EngineType[] memory);
    function processDepositCollateral(IEndpoint.DepositCollateral calldata txx) external;
    function registerProduct(uint32 _healthGroup, address _bookAddress, RiskHelper.ProductRisk memory _riskStore) external returns (uint32);
}
