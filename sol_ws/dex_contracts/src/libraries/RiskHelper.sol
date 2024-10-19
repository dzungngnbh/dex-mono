// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "../common/Errors.sol";
import "../interfaces/IEndpoint.sol";
import "./MathSD21x18.sol";

library RiskHelper {
    using MathSD21x18 for int128;

    // TODO: just move to Iendpoint
    struct ProductRisk {
        // these weights are all
        // between 0 and 2
        // these integers are the real
        // weights times 1e9
        int32 longWeightInitial;
        int32 shortWeightInitial;
        int32 longWeightMaintenance;
        int32 shortWeightMaintenance;
        int32 largePositionPenalty;
    }

    function _checkValidProductRisk(ProductRisk memory productRisk) internal pure {
        require(
            productRisk.longWeightInitial <= productRisk.longWeightMaintenance &&
            productRisk.shortWeightInitial >= productRisk.shortWeightMaintenance,
            ERR_BAD_RISK_STORE
        );
    }

    function _getWeightX18(ProductRisk memory productRisk, int128 amount, IEndpoint.HealthType healthType) internal view returns (int128 weight) {
        if (amount >= 0) {
            weight = healthType == IEndpoint.HealthType.INITIAL ? productRisk.longWeightInitial : productRisk.longWeightMaintenance;
        } else {
            weight = healthType == IEndpoint.HealthType.INITIAL ? productRisk.shortWeightInitial : productRisk.shortWeightMaintenance;
        }

        // check large position penalty
    }

    function _spotHealth(IEndpoint.HealthVars memory healthVars, int128 spotAmount) internal view returns (int128 health) {
        health = 0;
    }
}