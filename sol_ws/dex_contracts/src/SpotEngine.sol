pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import "./interfaces/engine/IProductEngine.sol";
import "./interfaces/engine/ISpotEngine.sol";

import "./BaseEngine.sol";
import "./SpotEngineState.sol";
import "./common/Constants.sol";
import "./ClearinghouseRisk.sol";

import "forge-std/console2.sol";

contract SpotEngine is BaseEngine, SpotEngineState {
    function initialize(
        address _endpointAddress,
        address _clearinghouseAddress,
        address _quote,
        address _owner
    ) external {
        require(_quote != address(0), "SpotEngine: quote address cannot be 0");
        _initialize(_endpointAddress, _clearinghouseAddress, _owner);

        // init quote asset
        configs[QUOTE_PRODUCT_ID] = Config({
            token: _quote,
            interestInflectionUtilX18: 8e17, // .8
            interestFloorX18: 1e16, // .01
            interestSmallCapX18: 4e16, // .04
            interestLargeCapX18: ONE
        });
        productStates[QUOTE_PRODUCT_ID] = BalanceState({
            cumulativeDepositsMultiplierX18: ONE,
            cumulativeBorrowsMultiplierX18: ONE,
            totalDepositsNormalized: 0,
            totalBorrowsNormalized: 0
        });
        // no need to init lp for QUOTE asset
        productIds.push(QUOTE_PRODUCT_ID);
    }

    function addProduct(
        uint32 healthGroup,
        address book,
        uint128 sizeIncrement,
        uint128 minSize,
        Config calldata config,
        RiskHelper.ProductRisk calldata productRisk)
    public onlyOwner {
        uint32 productId = _addProduct(healthGroup, book, sizeIncrement, minSize, productRisk);
        configs[productId] = config;
        _setDefaultState(productId);
    }

    function _setDefaultState(uint32 productId) internal {
        productStates[productId] = BalanceState({
            cumulativeDepositsMultiplierX18: ONE,
            cumulativeBorrowsMultiplierX18: ONE,
            totalDepositsNormalized: 0,
            totalBorrowsNormalized: 0
        });

        lpStates[productId] = LpState({
            supply: 0,
            quote: Balance({amount: 0, lastCumulativeMultiplierX18: ONE}),
            base: Balance({amount: 0, lastCumulativeMultiplierX18: ONE})
        });
    }

    // utils
    function getEngineType() external pure returns (EngineType) {
        return EngineType.SPOT;
    }

    function getConfig(uint32 productId) external view returns (Config memory) {
        return configs[productId];
    }

    function applyDeltas(ProductDelta[] calldata deltas) external canApplyDeltasOnly {
        for (uint32 i = 0; i < deltas.length; i++) {
            ProductDelta memory delta = deltas[i];

            if (delta.amountDelta == 0) { continue; }

            uint32 productId = delta.productId;
            bytes32 subaccount = delta.subaccount;
            int128 amountDelta = delta.amountDelta;

            BalanceState memory productState = productStates[productId];
            int128 balance = balances[productId][subaccount].balance;
            int128 newBalance = _updateBalanceNormalized(productState, balance, amountDelta);

            productStates[productId].totalDepositsNormalized = productState.totalDepositsNormalized;
            productStates[productId].totalBorrowsNormalized = productState.totalBorrowsNormalized;
            balances[productId][subaccount].balance = newBalance;
        }
    }

    // TODO: Init fee in the deployer
    function getWithdrawFee(uint32 productId) external view returns (int128) {
        return withdrawFees[productId];
    }
}