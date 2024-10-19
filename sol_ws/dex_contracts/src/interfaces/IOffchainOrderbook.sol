// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "./IEndpoint.sol";

interface IOffchainOrderbook {
    struct MarketConfig {
        uint32 productId;
        int128 collectedFees;
        int128 sequencerFees;
        int128 sizeIncrement;
        int128 priceIncrementX18;
        int128 lpSpreadX18;
        int128 minSize;
    }

    function initialize(address _endpointAddress, address _clearinghouseAddress, address _engineAddress, address _owner) external;

    function updateMarketConfig(int128 _sizeIncrement, int128 _priceIncrementX18, int128 _lpSpreadX18, int128 _minSize) external;

    function getDigest(IEndpoint.Order memory order) external view returns (bytes32);
    function getDomain() external view returns (bytes32);
}
