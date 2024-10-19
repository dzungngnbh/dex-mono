// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

import "./ClearinghouseRisk.sol";
import "./ClearinghouseState.sol";
import "./EndpointGated.sol";
import "./IClearinghouseEvents.sol";
import "./Version.sol";
import "./interfaces/IClearinghouse.sol";
import "./interfaces/IERC20Base.sol";
import "./interfaces/IEndpoint.sol";
import "./interfaces/engine/ISpotEngine.sol";
import "./libraries/ERC20Helper.sol";
import "./libraries/PriceHelper.sol";
import "./libraries/RiskHelper.sol";

contract Clearinghouse is UUPSUpgradeable, EndpointGated, IClearinghouse, IClearinghouseEvents, ClearinghouseRisk, ClearinghouseState, Version {

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() { _disableInitializers(); }

    function initialize(
        address _endpoint, // DI
        address _quote
    ) initializer public {
        __Ownable_init(msg.sender);
        __UUPSUpgradeable_init();
        numProducts = 1;

        quote = _quote;
        setEndpoint(_endpoint);

        productRisks[QUOTE_PRODUCT_ID] = RiskHelper.ProductRisk({
            longWeightInitial: 1e9,
            shortWeightInitial: 1e9,
            longWeightMaintenance: 1e9,
            shortWeightMaintenance: 1e9,
            largePositionPenalty: 0
        });
    }

    /// @dev required by the OZ UUPS module
    function _authorizeUpgrade(address) internal override onlyOwner {}

    /// @dev productEngine will be initialized inside addEngine, not at deployment
    function addEngine(address _engineAddress, IProductEngine.EngineType _engineType) external onlyOwner
    {
        require(_engineAddress != address(0), ERR_INVALID_ADDR);
        require(address(engineAddressByType[_engineType]) == address(0), "Clearinghouse: engine already exists");
        IProductEngine productEngine = IProductEngine(_engineAddress);

        // Register
        supportedEngines.push(_engineType);
        engineAddressByType[_engineType] = _engineAddress;

        // SPOT engine will have QUOTE_PRODUCT_ID
        // TODO: you can customize to have native token as quote, and this product cannot have perp yet,
        // since we only support perp with QUOTE_PRODUCT_ID 1 USDC token.
        if (_engineType == IProductEngine.EngineType.SPOT) {
            productIdToEngine[QUOTE_PRODUCT_ID] = productEngine;
        }

        // initilize
        address owner = owner();
        productEngine.initialize(
            getEndpoint(),
            address(this),
            quote,
            owner
        );
    }

    /// @dev Function is being called by IProductEngine only
    function registerProduct(
        uint32 _healthGroup,
        address _bookAddress,
        RiskHelper.ProductRisk memory _riskStore
    ) external returns (uint32) {
        IProductEngine productEngine = IProductEngine(msg.sender);
        IProductEngine.EngineType engineType = productEngine.getEngineType();
        require(address(engineAddressByType[engineType]) == msg.sender, ERR_UNAUTHORIZED);

        numProducts++;
        uint32 productId = getProductId(_healthGroup, engineType);
        productRisks[productId] = _riskStore;
        productIdToEngine[productId] = productEngine;
        IEndpoint(getEndpoint()).setProductIdBookAddress(productId, _bookAddress);

        return productId;
    }

    // processing transaction
    function processDepositCollateral(IEndpoint.DepositCollateral calldata txx) external onlyEndpoint {
        require(txx.amount < INT128_MAX, ERR_CONVERSION_OVERFLOW);
        ISpotEngine spotEngine = ISpotEngine(engineAddressByType[IProductEngine.EngineType.SPOT]);
        IERC20Base token = IERC20Base(spotEngine.getConfig(txx.productId).token);
        require(address(token) != address(0), ERR_INVALID_ADDR);
        require(token.decimals() <= MAX_DECIMALS);

        ERC20Helper.safeTransferFrom(token, msg.sender, address(this), uint256(int(txx.amount)));
        int128 chainAmount = PriceHelper.convertToChainAmount(int128(txx.amount), uint128(token.decimals()));

        ISpotEngine.ProductDelta [] memory deltas = new ISpotEngine.ProductDelta[](1);
        deltas[0] = ISpotEngine.ProductDelta({
            productId: txx.productId,
            subaccount: txx.subaccount,
            amountDelta: chainAmount,
            vQuoteDelta: 0
        });
        spotEngine.applyDeltas(deltas);
        emit UpdateCollateral(txx.productId, txx.subaccount, txx.amount);
    }

    // get total health for subaccount
    // TODO: We should get quote product id from clearinghouse, since we will have other markets beside USD[X]
    function getHealth(bytes32 subaccount, IEndpoint.HealthType healthType) external view returns (int128 health) {
        ISpotEngine spotEngine = ISpotEngine(engineAddressByType[IProductEngine.EngineType.SPOT]);
        ISpotEngine.SubaccountBalance memory quoteBalance = spotEngine.getBalance(subaccount, QUOTE_PRODUCT_ID);
        health = quoteBalance.balance;

        // TODO: update 1 to be something else, we need to redesign the healthGroup thing
        for(uint32 i = 0; i < 1; ++i) {
            IEndpoint.HealthVars memory healthVars;
            uint32 spotProductId = i*2+1;

            ISpotEngine.SubaccountBalance memory spotBalance = spotEngine.getBalance(subaccount, spotProductId);
            if (spotBalance.balance != 0 || spotBalance.lpBalance != 0) {
                healthVars.spotAmount = spotBalance.balance;
                healthVars.spotRisk = _getProductRisk(spotProductId);
            }

        }
    }

    function getProductId(uint32 healthGroup, IProductEngine.EngineType engineType) public pure returns (uint32 productId) {
        productId = healthGroup * 2 + 1;
        if (engineType == IProductEngine.EngineType.PERP) {
            productId++;
        }
    }

    function getEngineByType(IProductEngine.EngineType engineType) public view returns (address) {
        return engineAddressByType[engineType];
    }

    // views
    function getSupportedEngines()
    external
    view
    returns (IProductEngine.EngineType[] memory)
    {
        return supportedEngines;
    }
}
