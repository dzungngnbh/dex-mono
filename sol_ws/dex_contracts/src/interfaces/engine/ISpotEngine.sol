// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "./IProductEngine.sol";
import "../../libraries/RiskHelper.sol";

interface ISpotEngine is IProductEngine {
    struct Config {
        address token;
        int128 interestInflectionUtilX18;
        int128 interestFloorX18;
        int128 interestSmallCapX18;
        int128 interestLargeCapX18;
    }

    struct BalanceState {
        int128 cumulativeDepositsMultiplierX18;
        int128 cumulativeBorrowsMultiplierX18;
        int128 totalDepositsNormalized;
        int128 totalBorrowsNormalized;
    }

    struct ProductDelta {
        uint32 productId;
        bytes32 subaccount;
        int128 amountDelta;
        int128 vQuoteDelta;
    }

    struct Balance {
        int128 amount;
        int128 lastCumulativeMultiplierX18;
    }

    struct SubaccountBalance {
        int128 balance; // normalized version
        int128 lpBalance;
    }

    struct LpState {
        int128 supply;
        Balance quote;
        Balance base;
    }

    // functions
    function applyDeltas(ProductDelta[] calldata deltas) external;
    function addProduct(
        uint32 healthGroup,
        address book,
        uint128 sizeIncrement,
        uint128 minSize,
        Config calldata config,
        RiskHelper.ProductRisk calldata productRisk) external;

    function getWithdrawFee(uint32 productId) external view returns (int128);
    function getConfig(uint32 productId) external view returns (Config memory);
    function getStatesAndBalances(uint32 productId, bytes32 subaccount) external view returns (BalanceState memory, LpState memory, SubaccountBalance memory);
    function getBalance(bytes32 subaccount, uint32 productId) external view returns (SubaccountBalance memory);
}
