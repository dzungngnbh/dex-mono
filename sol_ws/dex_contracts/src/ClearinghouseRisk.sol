// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "./interfaces/IEndpoint.sol";
import "./libraries/MathSD21x18.sol";
import "./libraries/RiskHelper.sol";

abstract contract ClearinghouseRisk {
    using MathSD21x18 for int128;

    mapping(uint32 => RiskHelper.ProductRisk) internal productRisks;
    function _getProductRisk(uint32 productId) internal view returns (RiskHelper.ProductRisk memory productRisk) {
        productRisk = productRisks[productId];
    }
}
