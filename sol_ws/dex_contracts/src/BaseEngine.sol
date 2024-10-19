// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import {EndpointGated} from "./EndpointGated.sol";
import "./ClearinghouseRisk.sol";
import "./common/Errors.sol";
import "./interfaces/IClearinghouse.sol";
import "./interfaces/IEndpoint.sol";
import "./interfaces/IOffchainOrderbook.sol";
import "./interfaces/engine/IProductEngine.sol";
import "./libraries/MathSD21x18.sol";
import "./libraries/RiskHelper.sol";

abstract contract BaseEngine is IProductEngine, EndpointGated {
    using MathSD21x18 for int128;

    IClearinghouse internal _iClearinghouse;

    mapping(address => bool) internal canApplyDeltas;
    uint32[] internal productIds;

    // events
    event BalanceUpdate(uint32 productId, bytes32 subaccount);
    event ProductUpdate(uint32 productId);

    function _initialize(
        address _endpointAddress,
        address _clearinghouseAddress,
        address _owner
    ) internal initializer {
        require(_endpointAddress != address(0), "BaseEngine: endpoint address cannot be 0");
        require(_clearinghouseAddress != address(0), "BaseEngine: clearinghouse address cannot be 0");

        __Ownable_init(_owner);
        endpointAddress = _endpointAddress;
        _iClearinghouse = IClearinghouse(_clearinghouseAddress);

        canApplyDeltas[_clearinghouseAddress] = true;
        canApplyDeltas[_endpointAddress] = true;
    }

    function _addProduct(
        uint32 _healthGroup,
        address _bookAddress,
        uint128 _sizeIncrement,
        uint128 _minSize,
        RiskHelper.ProductRisk calldata _riskStore)
    internal onlyOwner returns (uint32) {
        require(_bookAddress != address(0), "BaseEngine: book address cannot be 0");
        RiskHelper._checkValidProductRisk(_riskStore);

        uint32 productId = _iClearinghouse.registerProduct(_healthGroup, _bookAddress, _riskStore);
        canApplyDeltas[_bookAddress] = true;
        address endpointAddress = getEndpoint();

        IOffchainOrderbook orderbook = IEndpoint(getEndpoint()).getProductOrderbook(productId);
        // init orderbook
        orderbook.initialize(
            endpointAddress,
            address(_iClearinghouse),
            address(this),
            owner()
        );

        return productId;
    }

    // utils
    function getProductIds() external view returns (uint32[] memory) {
        return productIds;
    }

    function _checkCanApplyDeltas() internal view virtual {
        require(canApplyDeltas[msg.sender], ERR_UNAUTHORIZED);
    }

    modifier canApplyDeltasOnly() {
        _checkCanApplyDeltas();
        _;
    }
}