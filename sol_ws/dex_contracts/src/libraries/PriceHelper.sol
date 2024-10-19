// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "../common/Constants.sol";

library PriceHelper {
    function convertToChainAmount(int128 _oracleAmount, uint128 _tokenDecimals) external returns (int128) {
        require(_tokenDecimals <= ORACLE_PRICE_DECIMALS, "invalid token decimals");
        int128 multiplier = int128(int(10**(ORACLE_PRICE_DECIMALS - _tokenDecimals)));
        return _oracleAmount / int128(multiplier);
    }
}