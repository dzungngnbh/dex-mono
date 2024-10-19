// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "./interfaces/engine/ISpotEngine.sol";
import "./libraries/MathSD21x18.sol";

abstract contract SpotEngineState is ISpotEngine {
    using MathSD21x18 for int128;

    mapping(uint32 => Config) internal configs; // mapping productId => Config
    mapping(uint32 => BalanceState) public productStates;
    mapping(uint32 => LpState) public lpStates; // mapping productId => LpState
    mapping(uint32 => int128) public withdrawFees; // mapping productId => withdrawFee
    mapping(uint32 => mapping(bytes32 => SubaccountBalance)) public balances; // mapping productId => mapping subaccount => Balance

    function _updateBalanceNormalized(BalanceState memory productState, int128 balance, int128 balanceDelta) internal returns (int128) {
        int128 cumulativeMultiplierX18;

        // pre adjusting we will add it back later
        if (balance > 0) {
            productState.totalDepositsNormalized -= balance;
            cumulativeMultiplierX18 = productState.cumulativeDepositsMultiplierX18;
        } else {
            productState.totalBorrowsNormalized += balance;
            cumulativeMultiplierX18 = productState.cumulativeBorrowsMultiplierX18;
        }

        // calculate the new balance
        int128 newBalance = balance.mul(cumulativeMultiplierX18) + balanceDelta;
        if (newBalance > 0) {
            cumulativeMultiplierX18 = productState.cumulativeDepositsMultiplierX18;
        } else {
            cumulativeMultiplierX18 = productState.cumulativeBorrowsMultiplierX18;
        }
        newBalance = newBalance.div(cumulativeMultiplierX18);

        // post adjusting
        if (newBalance > 0) {
            productState.totalDepositsNormalized += newBalance;
        } else {
            productState.totalBorrowsNormalized -= newBalance;
        }

        return newBalance;
    }

    // view
    function getBalance(bytes32 subaccount, uint32 productId) external view returns (SubaccountBalance memory subaccountBalance) {
        subaccountBalance = balances[productId][subaccount];
    }

    // including productState, lpState and banalcne
    function getStatesAndBalances(uint32 productId, bytes32 subaccount) external view returns (BalanceState memory balanceState, LpState memory lpState, SubaccountBalance memory subaccountBalance) {
        balanceState = productStates[productId];
        lpState = lpStates[productId];
        subaccountBalance = balances[productId][subaccount];
    }

}