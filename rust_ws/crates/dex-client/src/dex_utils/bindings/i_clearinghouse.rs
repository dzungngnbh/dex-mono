pub use i_clearinghouse::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod i_clearinghouse {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addEngine"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addEngine"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_engine"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_engineType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum IProductEngine.EngineType",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getEngineByType"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getEngineByType"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("engineType"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("enum IProductEngine.EngineType",),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getHealth"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getHealth"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("healthType"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("enum IEndpoint.HealthType",),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("health"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSupportedEngines"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSupportedEngines",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "enum IProductEngine.EngineType[]",
                                ),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_endpoint"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_quote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("processDepositCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("processDepositCollateral",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txx"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct IEndpoint.DepositCollateral",
                                ),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerProduct"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerProduct"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_healthGroup"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_bookAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_riskStore"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct RiskHelper.ProductRisk",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ICLEARINGHOUSE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct IClearinghouse<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IClearinghouse<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IClearinghouse<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IClearinghouse<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IClearinghouse<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IClearinghouse))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IClearinghouse<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ICLEARINGHOUSE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `addEngine` (0x1750893e) function
        pub fn add_engine(
            &self,
            engine: ::ethers::core::types::Address,
            engine_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 80, 137, 62], (engine, engine_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getEngineByType` (0x5d2e9ad1) function
        pub fn get_engine_by_type(
            &self,
            engine_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([93, 46, 154, 209], engine_type)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getHealth` (0x88b6496f) function
        pub fn get_health(
            &self,
            subaccount: [u8; 32],
            health_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, i128> {
            self.0
                .method_hash([136, 182, 73, 111], (subaccount, health_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSupportedEngines` (0xf5c662ab) function
        pub fn get_supported_engines(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u8>> {
            self.0
                .method_hash([245, 198, 98, 171], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x485cc955) function
        pub fn initialize(
            &self,
            endpoint: ::ethers::core::types::Address,
            quote: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 92, 201, 85], (endpoint, quote))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processDepositCollateral` (0x53e07362) function
        pub fn process_deposit_collateral(
            &self,
            txx: DepositCollateral,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 224, 115, 98], (txx,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerProduct` (0x40810a20) function
        pub fn register_product(
            &self,
            health_group: u32,
            book_address: ::ethers::core::types::Address,
            risk_store: ProductRisk,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([64, 129, 10, 32], (health_group, book_address, risk_store))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IClearinghouse<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `addEngine` function with signature `addEngine(address,uint8)` and selector `0x1750893e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "addEngine", abi = "addEngine(address,uint8)")]
    pub struct AddEngineCall {
        pub engine: ::ethers::core::types::Address,
        pub engine_type: u8,
    }
    ///Container type for all input parameters for the `getEngineByType` function with signature `getEngineByType(uint8)` and selector `0x5d2e9ad1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getEngineByType", abi = "getEngineByType(uint8)")]
    pub struct GetEngineByTypeCall {
        pub engine_type: u8,
    }
    ///Container type for all input parameters for the `getHealth` function with signature `getHealth(bytes32,uint8)` and selector `0x88b6496f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getHealth", abi = "getHealth(bytes32,uint8)")]
    pub struct GetHealthCall {
        pub subaccount: [u8; 32],
        pub health_type: u8,
    }
    ///Container type for all input parameters for the `getSupportedEngines` function with signature `getSupportedEngines()` and selector `0xf5c662ab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSupportedEngines", abi = "getSupportedEngines()")]
    pub struct GetSupportedEnginesCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address)` and selector `0x485cc955`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address)")]
    pub struct InitializeCall {
        pub endpoint: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `processDepositCollateral` function with signature `processDepositCollateral((bytes32,uint32,int128))` and selector `0x53e07362`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "processDepositCollateral",
        abi = "processDepositCollateral((bytes32,uint32,int128))"
    )]
    pub struct ProcessDepositCollateralCall {
        pub txx: DepositCollateral,
    }
    ///Container type for all input parameters for the `registerProduct` function with signature `registerProduct(uint32,address,(int32,int32,int32,int32,int32))` and selector `0x40810a20`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "registerProduct",
        abi = "registerProduct(uint32,address,(int32,int32,int32,int32,int32))"
    )]
    pub struct RegisterProductCall {
        pub health_group: u32,
        pub book_address: ::ethers::core::types::Address,
        pub risk_store: ProductRisk,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum IClearinghouseCalls {
        AddEngine(AddEngineCall),
        GetEngineByType(GetEngineByTypeCall),
        GetHealth(GetHealthCall),
        GetSupportedEngines(GetSupportedEnginesCall),
        Initialize(InitializeCall),
        ProcessDepositCollateral(ProcessDepositCollateralCall),
        RegisterProduct(RegisterProductCall),
    }
    impl ::ethers::core::abi::AbiDecode for IClearinghouseCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddEngineCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddEngine(decoded));
            }
            if let Ok(decoded) =
                <GetEngineByTypeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetEngineByType(decoded));
            }
            if let Ok(decoded) = <GetHealthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetHealth(decoded));
            }
            if let Ok(decoded) =
                <GetSupportedEnginesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSupportedEngines(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <ProcessDepositCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessDepositCollateral(decoded));
            }
            if let Ok(decoded) =
                <RegisterProductCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RegisterProduct(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IClearinghouseCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddEngine(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetEngineByType(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetHealth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSupportedEngines(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProcessDepositCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RegisterProduct(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IClearinghouseCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddEngine(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetEngineByType(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHealth(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSupportedEngines(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessDepositCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterProduct(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddEngineCall> for IClearinghouseCalls {
        fn from(value: AddEngineCall) -> Self {
            Self::AddEngine(value)
        }
    }
    impl ::core::convert::From<GetEngineByTypeCall> for IClearinghouseCalls {
        fn from(value: GetEngineByTypeCall) -> Self {
            Self::GetEngineByType(value)
        }
    }
    impl ::core::convert::From<GetHealthCall> for IClearinghouseCalls {
        fn from(value: GetHealthCall) -> Self {
            Self::GetHealth(value)
        }
    }
    impl ::core::convert::From<GetSupportedEnginesCall> for IClearinghouseCalls {
        fn from(value: GetSupportedEnginesCall) -> Self {
            Self::GetSupportedEngines(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for IClearinghouseCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<ProcessDepositCollateralCall> for IClearinghouseCalls {
        fn from(value: ProcessDepositCollateralCall) -> Self {
            Self::ProcessDepositCollateral(value)
        }
    }
    impl ::core::convert::From<RegisterProductCall> for IClearinghouseCalls {
        fn from(value: RegisterProductCall) -> Self {
            Self::RegisterProduct(value)
        }
    }
    ///Container type for all return fields from the `getEngineByType` function with signature `getEngineByType(uint8)` and selector `0x5d2e9ad1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetEngineByTypeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getHealth` function with signature `getHealth(bytes32,uint8)` and selector `0x88b6496f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetHealthReturn {
        pub health: i128,
    }
    ///Container type for all return fields from the `getSupportedEngines` function with signature `getSupportedEngines()` and selector `0xf5c662ab`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSupportedEnginesReturn(pub ::std::vec::Vec<u8>);
    ///Container type for all return fields from the `registerProduct` function with signature `registerProduct(uint32,address,(int32,int32,int32,int32,int32))` and selector `0x40810a20`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RegisterProductReturn(pub u32);
    ///`DepositCollateral(bytes32,uint32,int128)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositCollateral {
        pub subaccount: [u8; 32],
        pub product_id: u32,
        pub amount: i128,
    }
    ///`ProductRisk(int32,int32,int32,int32,int32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProductRisk {
        pub long_weight_initial: i32,
        pub short_weight_initial: i32,
        pub long_weight_maintenance: i32,
        pub short_weight_maintenance: i32,
        pub large_position_penalty: i32,
    }
}
