// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import "./EndpointGated.sol";
import "./Version.sol";
import "./common/Errors.sol";
import "./interfaces/IClearinghouse.sol";
import "./interfaces/IOffchainOrderbook.sol";
import "./interfaces/engine/IProductEngine.sol";
import "./libraries/MathHelper.sol";

import "forge-std/console2.sol";

contract OffchainOrderbook is EIP712Upgradeable, UUPSUpgradeable, OwnableUpgradeable, IOffchainOrderbook, EndpointGated, Version {
    IProductEngine public productEngine;
    IClearinghouse public iClearinghouse;

    // "Order(bytes32 sender,int128 priceX18,int128 amount,uint64 expiration,uint64 nonce)"
    bytes32 constant internal ORDER_TYPE_HASH = 0x4b5dde172732338a171d63d3b949aacf6bb234046f1395e1986341aad20e7d48;

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {_disableInitializers();}

    function initialize(
        address _endpointAddress,
        address _clearinghouseAddress,
        address _engineAddress,
        address _owner
    ) initializer public {
        __Ownable_init(_owner);
        __UUPSUpgradeable_init();
        __EIP712_init("DEX", "0.1.0");

        endpointAddress = _endpointAddress;
        iClearinghouse = IClearinghouse(_clearinghouseAddress);
        productEngine = IProductEngine(_engineAddress);
    }

    ///@dev required by the OZ UUPS module
    function _authorizeUpgrade(address) internal override onlyOwner {}

    function matchOrders(IEndpoint.OrderMatchingWithSigner calldata txn) external onlyEndpoint {
        IEndpoint.SignedOrder memory takerOrder = txn.orders.takerOrder;
        IEndpoint.SignedOrder memory makerOrder = txn.orders.makerOrder;
        require(_isValidOrder(takerOrder.order) && _isValidOrder(makerOrder.order), ERR_INVALID_ORDER);

        // ensure orders are crossing
        require((takerOrder.order.amount > 0) != (makerOrder.order.amount > 0), ERR_ORDERS_CANNOT_BE_MATCHED);
        if (makerOrder.order.amount > 0) {
            require(takerOrder.order.priceX18 >= makerOrder.order.priceX18, ERR_ORDERS_CANNOT_BE_MATCHED);
        } else {
            require(takerOrder.order.priceX18 <= makerOrder.order.priceX18, ERR_ORDERS_CANNOT_BE_MATCHED);
        }

        int128 takerAmount = takerOrder.order.amount;
    }

    function _matchOrder(
        IEndpoint.Order memory takerOrder,
        IEndpoint.Order memory makerOrder
    ) internal returns (int128 takerAmountDelta, int128 takerQuoteDelta) {
        if (takerOrder.amount > 0) {
            takerAmountDelta = MathHelper.max(takerOrder.amount, -makerOrder.amount);
        } else {
            takerAmountDelta = MathHelper.min(takerOrder.amount, -makerOrder.amount);
        }
        // the case .amount = 0 is not possible, since we already checked in _isValidOrder

//        takerAmountDelta = takerAmountDelta % sizeIcrement;
        int128 makerAmountDelta = takerAmountDelta * makerOrder.priceX18;
        takerQuoteDelta = -makerAmountDelta;

        // apply fee to maker
        
    }

    // TODO: implement
    function _isValidOrder(IEndpoint.Order memory order) internal view returns (bool) {
        // fill partially?

        // we ignore the part of validating the signature and order
        // it will save gas cost, all submitted orders are being checked offchain
        // visit: dex/sequencer to search and validate submitted orders.
        return order.amount != 0 && order.priceX18 > 0 && !_isExpiredOrder(order.expiration);
    }

    function _isExpiredOrder(uint64 expiration) internal view returns (bool) {
        return expiration & ((1 << 58) - 1) <= getOracleTime();
    }

    function getOracleTime() public view returns (uint128) {
        return IEndpoint(getEndpoint()).getOracleTime();
    }

    function getDigest(IEndpoint.Order memory order) public view returns (bytes32) {
        return _hashTypedDataV4(keccak256(abi.encode(ORDER_TYPE_HASH, order)));
    }

    function getDomain() public view returns (bytes32) {
        return _domainSeparatorV4();
    }

    function _requireValidSignature(bytes32 subaccount, bytes32 digest, bytes memory signature) internal view {
        address signer = ECDSA.recover(digest, signature);
        require(signer == address(0) && signer == address(bytes20(subaccount)),
            ERR_UNAUTHORIZED);
    }

    function updateMarketConfig(int128 _sizeIncrement, int128 _priceIncrementX18, int128 _lpSpreadX18, int128 _minSize) external {
    }
}