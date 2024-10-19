// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "./IOffchainOrderbook.sol";
import "../libraries/RiskHelper.sol";

interface IEndpoint {
    event SubmitTransactions();

    enum TransactionType {
        DepositCollateral,
        WithdrawCollateral,
        SpotTick,
        UpdatePrice
    }

    // order handling types
    // todo: add prefix Params
    struct DepositCollateral {
        bytes32 subaccount;
        uint32 productId;
        int128 amount;
    }

    struct UpdatePriceParams {
        uint32 healthGroup;
        int128 spotPriceX18;
        int128 perpPriceX18;
    }

    // order matching
    struct Order {
        bytes32 sender;
        int128 priceX18;
        int128 amount; // positive is buy, negative is sell
        uint64 expiration;
        uint64 nonce;
    }

    struct SignedOrder {
        Order order;
        bytes signature; // signed hash of order
    }

    struct MatchedOrders {
        uint32 productId;
        SignedOrder takerOrder;
        SignedOrder makerOrder;
    }

    struct OrderMatchingWithSigner {
        MatchedOrders orders;
        bytes32 subAccountTaker;
        bytes32 subAccountMaker;
    }

    // other things
    struct PricesX18 {
        int128 spotPriceX18;
        // int128 perpPriceX18;
    }

    struct Times {
        uint128 spotTime;
        uint128 perpTime;
    }

    // risk
    enum HealthType {
        INITIAL,
        MAINTENANCE
    }

    struct HealthGroup {
        uint32 spotId;
        uint32 perpId;
    }

    // This HealthVars is being used to calculate the health of the account
    struct HealthVars {
        int128 spotAmount;
        int128 perpAmount;
        // 1 unit of basis amount is 1 unit long spot and 1 unit short perp
        int128 basisAmount;
        int128 spotInLpAmount;
        int128 perpInLpAmount;
        PricesX18 pricesX18;
        RiskHelper.ProductRisk spotRisk;
//        RiskHelper.Risk perpRisk;
    }

    // account interacting functions directly
    function depositCollateral(bytes12 subaccountName, uint32 productId, uint128 amount) external;

    function getNonce(address _sender) external view returns (uint64);
    function getOracleTime() external view returns (uint128);
    function getProductOrderbook(uint32 _productId) external view returns (IOffchainOrderbook);
    function getSequencerAddress() external view returns (address);
    function setProductIdBookAddress(uint32 _productId, address _bookAddress) external;
    function setSequencerAddress(address _sequencerAddress) external;
    function submitTransactions(bytes[] calldata txs) external;
}
