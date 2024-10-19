// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

interface IProductEngine {
    enum EngineType {
        SPOT,
        PERP
    }

    function initialize(address _endpoint, address _clearinghouseAddress, address _quote, address _owner) external;
    function getEngineType() external pure returns (EngineType);
    function getProductIds() external view returns (uint32[] memory);

//    function swapLp(
//        uint32 productId,
//        int128 amount,
//        int128 priceX18,
//        int128 sizeIncrement,
//        int128 lpSpreadX18
//    ) external returns (int128, int128);
//
//    function swapLp(
//        uint32 productId,
//        int128 baseDelta,
//        int128 quoteDelta
//    ) external returns (int128, int128);
//
//    function mintLp(
//        uint32 productId,
//        bytes32 subaccount,
//        int128 amountBase,
//        int128 quoteAmountLow,
//        int128 quoteAmountHigh
//    ) external;
//
//    function burnLp(
//        uint32 productId,
//        bytes32 subaccount,
//    // passing 0 here means to burn all
//        int128 amountLp
//    ) external returns (int128, int128);
}
