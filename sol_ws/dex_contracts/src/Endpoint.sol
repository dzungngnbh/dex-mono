// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "./Version.sol";
import "./interfaces/IClearinghouse.sol";
import "./interfaces/IERC20Base.sol";
import "./interfaces/IEndpoint.sol";
import "./interfaces/IOffchainOrderbook.sol";
import "./interfaces/ISanctionsList.sol";
import "./interfaces/engine/ISpotEngine.sol";
import "./libraries/ERC20Helper.sol";

import "forge-std/console2.sol";

contract Endpoint is IEndpoint, EIP712Upgradeable, UUPSUpgradeable, OwnableUpgradeable, Version {
    address internal sequencer;
    int128 public sequencerFees;

    // storage
    mapping(uint32 => address) internal productIdToBookAddress;
    mapping(address => string) public referralCodes;
    mapping(address => uint64) internal nonces;
    mapping(uint32 => IEndpoint.PricesX18) internal healthGroupPricesX18;

    IEndpoint.Times internal times;
    IClearinghouse public clearinghouse;
    ISanctionsList private sanctions;
    ISpotEngine private spotEngine;

    // events
    event DepositCollateralEvent();

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {_disableInitializers();}

    // @dev: clearinghouse must be init and addEngine before calling this
    function initialize(
        address _sequencer, // our own sequencer
        address _clearinghouse,
        uint128 _time
    ) initializer public {
        require(_time != 0, ERR_INVALID_TIME);
        __EIP712_init("DEX", "0.1.0");
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();

        times = Times({perpTime: _time, spotTime: _time});
        sequencer = _sequencer;
        clearinghouse = IClearinghouse(_clearinghouse);
        spotEngine = ISpotEngine(clearinghouse.getEngineByType(IProductEngine.EngineType.SPOT));
    }

    ///@dev required by the OZ UUPS module
    function _authorizeUpgrade(address) internal override onlyOwner {}

    function depositCollateral(
        bytes12 subaccountName,
        uint32 productId,
        uint128 amount
    ) external {
        depositCollateralWithReferral(
            bytes32(abi.encodePacked(msg.sender, subaccountName)),
            productId,
            amount,
            DEFAULT_REFERRAL_CODE
        );
        emit DepositCollateralEvent();
    }

    function depositCollateralWithReferral(
        bytes32 subaccount,
        uint32 productId,
        uint128 amount,
        string memory referralCode
    ) public {
        require(bytes(referralCode).length != 0, ERR_INVALID_REFERRAL_CODE);

        address sender = address(bytes20(subaccount));

        // depositor / depositee need to be unsanctioned
//        requireUnsanctioned(msg.sender);
//        requireUnsanctioned(sender);

        // no referral code allowed for remote deposit
        setReferralCode(
            sender,
            sender == msg.sender ? referralCode : DEFAULT_REFERRAL_CODE
        );

        IERC20Base token = IERC20Base(spotEngine.getConfig(productId).token);
        require(address(token) != address(0));
        handleDepositTransfer(token, msg.sender, uint256(amount));

        DepositCollateral memory depositCollateral_ = DepositCollateral({
            subaccount: subaccount,
            productId: productId,
            amount: int128(amount)
        });
        _processDepositCollateral(depositCollateral_);
        // and handle deposit from clearinghouse as well
        // ignore slow mode
    }

    function setReferralCode(address sender, string memory referralCode)
    internal
    {
        if (bytes(referralCodes[sender]).length == 0) {
            referralCodes[sender] = referralCode;
        }
    }

    function setProductIdBookAddress(uint32 _productId, address _bookAddress) external onlyClearinghouse {
        productIdToBookAddress[_productId] = _bookAddress;
    }

    function getProductOrderbook(uint32 _productId) external view returns (IOffchainOrderbook) {
        return IOffchainOrderbook(productIdToBookAddress[_productId]);
    }

    function getBookAddress(uint32 _productId) external view returns (address) {
        return productIdToBookAddress[_productId];
    }

    function handleDepositTransfer(
        IERC20Base token,
        address from,
        uint256 amount
    ) internal {
        ERC20Helper.safeIncreaseAllowance(token, address(clearinghouse), amount);
        ERC20Helper.safeTransferFrom(token, from, address(this), amount);
    }

    function submitTransactions(
        bytes[] calldata txs
    ) external onlySequencer {
        // TODO: We should have a way to check if the transactions are processed properly
        for (uint128 i = 0; i < txs.length; i++) {
            _processTx(txs[i]);
        }
    }

    function _processTx(bytes calldata transaction) private {
        TransactionType txType = TransactionType(uint8(transaction[0]));

        if (txType == TransactionType.WithdrawCollateral) {
            _processWithdrawCollateral(transaction);
        } else if (txType == TransactionType.UpdatePrice) {
            _updatePrice(transaction);
        } else {
            revert(ERR_INVALID_TX_TYPE);
        }
    }

    // this function is still kept for client to access through client
    function _processDepositCollateral(DepositCollateral memory txx) private {
//        validateSender(txx.subaccount, msg.sender);
        // TODO: validate the case that sender hasn't deposited and still can trade
        clearinghouse.processDepositCollateral(txx);
    }

    function _processWithdrawCollateral(bytes calldata transaction) private {
        revert();
    }

    function _updatePrice(bytes calldata transaction) internal {
        UpdatePriceParams memory updatePriceParams = abi.decode(transaction[1:], (UpdatePriceParams));
        require(updatePriceParams.spotPriceX18 != 0, ERR_INVALID_PRICE);
        require(updatePriceParams.perpPriceX18 != 0, ERR_INVALID_PRICE);

        healthGroupPricesX18[updatePriceParams.healthGroup].spotPriceX18 = updatePriceParams.spotPriceX18;
        // healthGroupPricesX18[updatePriceParams.healthGroup].perpPriceX18 = updatePriceParams.perpPriceX18;
    }

    function getOracleTime() external view returns (uint128) {
        uint128 _time = times.perpTime > times.spotTime ? times.perpTime : times.spotTime;
        require(_time != 0, ERR_INVALID_TIME);
        return _time;
    }

    // utils
    function setSequencerAddress(address _sequencerAddress) external onlyOwner {
        require(_sequencerAddress != address(0), ERR_INVALID_ADDR);
        sequencer = _sequencerAddress;
    }

    function getSequencerAddress() external view returns (address) {
        return sequencer;
    }

    function isValidNonce(address _sender, uint64 _nonce) internal view returns (bool) {
        return _nonce == nonces[_sender] + 1;
    }

    function getNonce(address _sender) external view returns (uint64) {
        return nonces[_sender];
    }

    modifier onlySequencer() {
        require(msg.sender == sequencer, ERR_ONLY_SEQUENCER);
        _;
    }

    modifier onlyClearinghouse() {
        require(msg.sender == address(clearinghouse), ERR_UNAUTHORIZED);
        _;
    }

    function requireUnsanctioned(address sender) internal view {
        require(!sanctions.isSanctioned(sender), ERR_WALLET_SANCTIONED);
    }

    function validateSender(bytes32 txSender, address sender) internal view {
        require(
            address(bytes20(txSender)) == sender ||
            sender == address(this),
            ERR_WRONG_SENDER
        );
    }
}