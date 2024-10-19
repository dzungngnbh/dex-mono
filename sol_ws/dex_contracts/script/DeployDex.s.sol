// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "../src/Clearinghouse.sol";
import "../src/Endpoint.sol";
import "../src/OffchainOrderbook.sol";
import "../src/SpotEngine.sol";
import "../src/interfaces/IERC20Base.sol";
import "../src/interfaces/IEndpoint.sol";
import "../src/libraries/RiskHelper.sol";
import "../src/common/Constants.sol";

contract DAIMock is ERC20 {
    constructor(uint256 initialSupply) ERC20("DAIMock", "DAI.M") {
        _mint(msg.sender, initialSupply);
    }
}

contract BTCMock is ERC20 {
    constructor(uint256 initialSupply) ERC20("BTCMock", "BTC.M") {
        _mint(msg.sender, initialSupply);
    }
}

contract DeployDexScript is Script {
    // on arbitrum
    address private constant ACC_8 = 0x8a85C375384aDE299eC4E367a812A08DF4303beD;
    address private constant MM_ADDRESSS = 0xEF7AdD5a2001c6f97D45141625670BB32c19425A;

    uint256 private constant ONE_X18 = 1e18;

    uint32 BTC_DAI_SPOT_PRODUCT_ID = 3;

    function setUp() public {}

    function run() public {
        // private input through --private-key=
        vm.startBroadcast();

        uint128 currentTime = uint128(block.timestamp);
        console2.log("Current time: ", currentTime);

        // deploy mocksToken
        (address daiMockAddress, address btcMockAddress) = _deployDexMockTokens();

        // 1. create clearinghouse
        IClearinghouse clearinghouse = new Clearinghouse();
        ERC1967Proxy clearinghouseProxy = new ERC1967Proxy(address(clearinghouse), "");
        IClearinghouse wrappedClearinghouse = IClearinghouse(address(clearinghouseProxy));
        address clearinghouseProxyAddr = address(clearinghouseProxy);

        // 2. create endpoint
        Endpoint endpoint = new Endpoint();
        ERC1967Proxy endpointProxy = new ERC1967Proxy(address(endpoint), "");
        Endpoint wrappedEndpoint = Endpoint(address(endpointProxy));

        address sequencerAddress = ACC_8;

        // 3. clearinghouse and deploy engines
        wrappedClearinghouse.initialize(address(endpointProxy), daiMockAddress);
        console2.log("Clearinghouse proxy initialized at address : ", clearinghouseProxyAddr);
        address spotEngineProxyAddress = _deploySpotEngine(clearinghouseProxyAddr); // addEngine to clearinghouse

        // 4. init endpoint
        wrappedEndpoint.initialize(sequencerAddress, clearinghouseProxyAddr, currentTime);
        console2.log("Endpoint proxy deployed at address      : ", address(endpointProxy));

        // 5. addProducts to engines
        _addProduct(spotEngineProxyAddress, btcMockAddress);

        // output
        console2.log("Sequencer address                       : ", sequencerAddress);

        vm.stopBroadcast();

        // sanity check
       _mmActions(wrappedEndpoint, btcMockAddress, daiMockAddress, spotEngineProxyAddress, sequencerAddress, clearinghouseProxyAddr);
    }

    function _deployDexMockTokens() internal returns (address, address) {
        // initial supply to be 1_000_000 each
        DAIMock daiMock = new DAIMock(1_000_000 * ONE_X18);
        BTCMock btcMock = new BTCMock(1_000_000 * ONE_X18);

        // mint token to market maker
        // assume BTC 40k, mint 400k DAI and 10 BTC
        daiMock.transfer(MM_ADDRESSS, 400_000 * ONE_X18);
        btcMock.transfer(MM_ADDRESSS, 10 * ONE_X18);

        // transfer to current account 
        daiMock.transfer(msg.sender, 400_000 * ONE_X18);
        btcMock.transfer(msg.sender, 10 * ONE_X18);
        

        console2.log("DAI.M mock deployed at address: ", address(daiMock));
        console2.log("BTC.M mock deployed at address: ", address(btcMock));

        return (address(daiMock), address(btcMock));
    }

    function _deploySpotEngine(address _clearinghouseProxy) internal returns (address) {
        IClearinghouse iClearinghouse = Clearinghouse(_clearinghouseProxy);

        // create spot engine
        SpotEngine spotEngine = new SpotEngine();
        ERC1967Proxy spotEngineProxy = new ERC1967Proxy(address(spotEngine), "");
        SpotEngine wrappedSpotEngine = SpotEngine(address(spotEngineProxy));

        // add and init engine, since the engine calling _init, then we need to init the
        // spotEngine with owner which is clearingHouse owner, not clearinghouse proxy msg.sender
        iClearinghouse.addEngine(address(spotEngineProxy), IProductEngine.EngineType.SPOT);
        console2.log("SpotEngine proxy deployed at address: ", address(spotEngineProxy));
        return address(spotEngineProxy);

    }

    function _addProduct(address _spotEngineProxy, address _btcMockAddress) internal {
        ISpotEngine wrappedSpotEngine = ISpotEngine(_spotEngineProxy);

        // add product
        uint32 healthGroup = 1;
        IOffchainOrderbook btcOffchainOrderbook = new OffchainOrderbook();
        ERC1967Proxy btcOffchainOrderbookProxy = new ERC1967Proxy(address(btcOffchainOrderbook), "");

        uint128 sizeIncrement = 0;
        uint128 minSize = 0;
        ISpotEngine.Config memory config = ISpotEngine.Config({
            token: _btcMockAddress,
            interestInflectionUtilX18: 8e17, // .8
            interestFloorX18: 1e16, // .01
            interestSmallCapX18: 4e16, // .04
            interestLargeCapX18: ONE
        });
        RiskHelper.ProductRisk memory productRisk = RiskHelper.ProductRisk({
            longWeightInitial: 1e9,
            shortWeightInitial: 1e9,
            longWeightMaintenance: 1e9,
            shortWeightMaintenance: 1e9,
            largePositionPenalty: 0
        });
        wrappedSpotEngine.addProduct(healthGroup, address(btcOffchainOrderbookProxy), sizeIncrement, minSize, config, productRisk);

        console2.log("btcOffchainOrderbookProxy deployed at address: ", address(btcOffchainOrderbookProxy));

        // playground
        IOffchainOrderbook btcOffchainOrderbookWrapped = IOffchainOrderbook(address(btcOffchainOrderbookProxy));
        bytes32 subaccount = _constructDefaultSubAccount(MM_ADDRESSS);
        console2.log("subaccount: ");
        console2.logBytes32(subaccount);
        IEndpoint.Order memory order = IEndpoint.Order({
            sender: subaccount,
            priceX18: 0,
            amount: 0,
            expiration: 0,
            nonce: 0
        });
    }

    function _mmActions(IEndpoint wrappedEndpoint, address btcMockAddress, address daiMockAddress, address _spotEngineProxyAddress, address _sequencerAddress, address _clearinghouseProxy) internal {
        uint256 MM_PRIVATE_KEY = vm.envUint("MM_PRIVATE_KEY");
        uint256 SEQUENCER_PRIVATE_KEY = vm.envUint("SEQUENCER_PRIVATE_KEY");

        vm.startBroadcast(MM_PRIVATE_KEY);
        console2.log("msg.sender: ", msg.sender);

        IERC20Base btcMock = IERC20Base(btcMockAddress);
        IERC20Base daiMock = IERC20Base(daiMockAddress);
        btcMock.approve(address(wrappedEndpoint), 10 * ONE_X18);
        daiMock.approve(address(wrappedEndpoint), 400_000 * ONE_X18);
        wrappedEndpoint.depositCollateral('default', BTC_DAI_SPOT_PRODUCT_ID, uint128(10 * ONE_X18));
        wrappedEndpoint.depositCollateral('default', QUOTE_PRODUCT_ID, uint128(400_000 * ONE_X18));

        uint256 leftBtc = btcMock.balanceOf(MM_ADDRESSS);
        uint256 leftDai = daiMock.balanceOf(MM_ADDRESSS);
        console2.log("leftBtc: ", leftBtc);
        console2.log("leftDai: ", leftDai);

        // test getting user balance for frontend
        bytes32 defaultMMSubAccount = _constructDefaultSubAccount(MM_ADDRESSS);
        ISpotEngine spotEngine = ISpotEngine(_spotEngineProxyAddress);
        ISpotEngine.SubaccountBalance memory mmBalance = spotEngine.getBalance(defaultMMSubAccount, BTC_DAI_SPOT_PRODUCT_ID);
        console2.log("mmBalance", mmBalance.balance);

        // get getHealth
        IClearinghouse clearinghouse = IClearinghouse(_clearinghouseProxy);
        int128 health = clearinghouse.getHealth(defaultMMSubAccount, IEndpoint.HealthType.INITIAL);
        console2.log("health typee: ", uint256(IEndpoint.HealthType.INITIAL));
        console2.log("health: ", health);

        vm.stopBroadcast();

        // output info
        _printSubaccountInfo(_spotEngineProxyAddress, _constructDefaultSubAccount(MM_ADDRESSS), BTC_DAI_SPOT_PRODUCT_ID);
        _printSubaccountInfo(_spotEngineProxyAddress, _constructDefaultSubAccount(MM_ADDRESSS), QUOTE_PRODUCT_ID);

        // placeOrder
        // playground
    }

    // helper
    function _printSubaccountInfo(address _spotEngineProxyAddress, bytes32 subaccount, uint32 productId) internal {
        ISpotEngine spotEngine = ISpotEngine(_spotEngineProxyAddress);
        (ISpotEngine.BalanceState memory balanceState, ISpotEngine.LpState memory lpState, ISpotEngine.SubaccountBalance memory subaccountBalance) = spotEngine.getStatesAndBalances(productId, _constructDefaultSubAccount(MM_ADDRESSS));

        console2.log("------------------------------------------");
        console2.log("productId info for subaccount: %s", productId);
        console2.log("balanceState.totalDepositsNormalized: ", balanceState.totalDepositsNormalized);
        console2.log("balanceState.totalBorrowsNormalized: ", balanceState.totalBorrowsNormalized);
        console2.log("subaccountBalance.balance: ", subaccountBalance.balance);

    }

    function _constructSubAccount(address sender, bytes12 subaccount) internal returns (bytes32) {
        return bytes32(abi.encodePacked(bytes20(sender), subaccount));
    }

    function _constructDefaultSubAccount(address sender) internal returns (bytes32) {
        return _constructSubAccount(sender, 'default');
    }
}
