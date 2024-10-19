pub use i_endpoint::*;

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
pub mod i_endpoint {
    use ethers::contract::Eip712;

    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("depositCollateral"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositCollateral"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("subaccountName"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(12usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes12"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("productId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("doSomething"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("doSomething"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNonce"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_sender"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getProductOrderbook"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getProductOrderbook",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_productId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IOffchainOrderbook",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSequencerAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSequencerAddress",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("setProductIdBookAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setProductIdBookAddress",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_productId"),
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
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setSequencerAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setSequencerAddress",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_sequencerAddress"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitTransactions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitTransactions"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txs"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("SubmitTransactions"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("SubmitTransactions"),
                    inputs: ::std::vec![],
                    anonymous: false,
                },],
            )]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }

    ///The parsed JSON ABI of the contract.
    pub static IENDPOINT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);

    pub struct IEndpoint<M>(::ethers::contract::Contract<M>);

    impl<M> ::core::clone::Clone for IEndpoint<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }

    impl<M> ::core::ops::Deref for IEndpoint<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<M> ::core::ops::DerefMut for IEndpoint<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<M> ::core::fmt::Debug for IEndpoint<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IEndpoint))
                .field(&self.address())
                .finish()
        }
    }

    impl<M: ::ethers::providers::Middleware> IEndpoint<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IENDPOINT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `depositCollateral` (0x8e5d588c) function
        pub fn deposit_collateral(
            &self,
            subaccount_name: [u8; 12],
            product_id: u32,
            amount: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 93, 88, 140], (subaccount_name, product_id, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `doSomething` (0xc189e283) function
        pub fn do_something(
            &self,
            p0: DepositCollateral,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 137, 226, 131], (p0,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0x2d0335ab) function
        pub fn get_nonce(
            &self,
            sender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([45, 3, 53, 171], sender)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getProductOrderbook` (0x58833c47) function
        pub fn get_product_orderbook(
            &self,
            product_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([88, 131, 60, 71], product_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSequencerAddress` (0x6875f178) function
        pub fn get_sequencer_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([104, 117, 241, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProductIdBookAddress` (0x7c80b105) function
        pub fn set_product_id_book_address(
            &self,
            product_id: u32,
            book_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 128, 177, 5], (product_id, book_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setSequencerAddress` (0xd5cb711b) function
        pub fn set_sequencer_address(
            &self,
            sequencer_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 203, 113, 27], sequencer_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitTransactions` (0x1f186b27) function
        pub fn submit_transactions(
            &self,
            txs: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 24, 107, 39], txs)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SubmitTransactions` event
        pub fn submit_transactions_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SubmitTransactionsFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SubmitTransactionsFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }

    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for IEndpoint<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }

    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SubmitTransactions", abi = "SubmitTransactions()")]
    pub struct SubmitTransactionsFilter;

    ///Container type for all input parameters for the `depositCollateral` function with signature `depositCollateral(bytes12,uint32,uint128)` and selector `0x8e5d588c`
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
        name = "depositCollateral",
        abi = "depositCollateral(bytes12,uint32,uint128)"
    )]
    pub struct DepositCollateralCall {
        pub subaccount_name: [u8; 12],
        pub product_id: u32,
        pub amount: u128,
    }

    ///Container type for all input parameters for the `doSomething` function with signature `doSomething((bytes32,uint32,int128))` and selector `0xc189e283`
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
    #[ethcall(name = "doSomething", abi = "doSomething((bytes32,uint32,int128))")]
    pub struct DoSomethingCall(pub DepositCollateral);

    ///Container type for all input parameters for the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
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
    #[ethcall(name = "getNonce", abi = "getNonce(address)")]
    pub struct GetNonceCall {
        pub sender: ::ethers::core::types::Address,
    }

    ///Container type for all input parameters for the `getProductOrderbook` function with signature `getProductOrderbook(uint32)` and selector `0x58833c47`
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
    #[ethcall(name = "getProductOrderbook", abi = "getProductOrderbook(uint32)")]
    pub struct GetProductOrderbookCall {
        pub product_id: u32,
    }

    ///Container type for all input parameters for the `getSequencerAddress` function with signature `getSequencerAddress()` and selector `0x6875f178`
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
    #[ethcall(name = "getSequencerAddress", abi = "getSequencerAddress()")]
    pub struct GetSequencerAddressCall;

    ///Container type for all input parameters for the `setProductIdBookAddress` function with signature `setProductIdBookAddress(uint32,address)` and selector `0x7c80b105`
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
        name = "setProductIdBookAddress",
        abi = "setProductIdBookAddress(uint32,address)"
    )]
    pub struct SetProductIdBookAddressCall {
        pub product_id: u32,
        pub book_address: ::ethers::core::types::Address,
    }

    ///Container type for all input parameters for the `setSequencerAddress` function with signature `setSequencerAddress(address)` and selector `0xd5cb711b`
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
    #[ethcall(name = "setSequencerAddress", abi = "setSequencerAddress(address)")]
    pub struct SetSequencerAddressCall {
        pub sequencer_address: ::ethers::core::types::Address,
    }

    ///Container type for all input parameters for the `submitTransactions` function with signature `submitTransactions(bytes[])` and selector `0x1f186b27`
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
    #[ethcall(name = "submitTransactions", abi = "submitTransactions(bytes[])")]
    pub struct SubmitTransactionsCall {
        pub txs: ::std::vec::Vec<::ethers::core::types::Bytes>,
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
    pub enum IEndpointCalls {
        DepositCollateral(DepositCollateralCall),
        DoSomething(DoSomethingCall),
        GetNonce(GetNonceCall),
        GetProductOrderbook(GetProductOrderbookCall),
        GetSequencerAddress(GetSequencerAddressCall),
        SetProductIdBookAddress(SetProductIdBookAddressCall),
        SetSequencerAddress(SetSequencerAddressCall),
        SubmitTransactions(SubmitTransactionsCall),
    }

    impl ::ethers::core::abi::AbiDecode for IEndpointCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DepositCollateralCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositCollateral(decoded));
            }
            if let Ok(decoded) = <DoSomethingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DoSomething(decoded));
            }
            if let Ok(decoded) = <GetNonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) =
                <GetProductOrderbookCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetProductOrderbook(decoded));
            }
            if let Ok(decoded) =
                <GetSequencerAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSequencerAddress(decoded));
            }
            if let Ok(decoded) =
                <SetProductIdBookAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProductIdBookAddress(decoded));
            }
            if let Ok(decoded) =
                <SetSequencerAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetSequencerAddress(decoded));
            }
            if let Ok(decoded) =
                <SubmitTransactionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitTransactions(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }

    impl ::ethers::core::abi::AbiEncode for IEndpointCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DepositCollateral(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DoSomething(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetProductOrderbook(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSequencerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProductIdBookAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSequencerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitTransactions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }

    impl ::core::fmt::Display for IEndpointCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::DoSomething(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetProductOrderbook(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSequencerAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProductIdBookAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSequencerAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitTransactions(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }

    impl ::core::convert::From<DepositCollateralCall> for IEndpointCalls {
        fn from(value: DepositCollateralCall) -> Self {
            Self::DepositCollateral(value)
        }
    }

    impl ::core::convert::From<DoSomethingCall> for IEndpointCalls {
        fn from(value: DoSomethingCall) -> Self {
            Self::DoSomething(value)
        }
    }

    impl ::core::convert::From<GetNonceCall> for IEndpointCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }

    impl ::core::convert::From<GetProductOrderbookCall> for IEndpointCalls {
        fn from(value: GetProductOrderbookCall) -> Self {
            Self::GetProductOrderbook(value)
        }
    }

    impl ::core::convert::From<GetSequencerAddressCall> for IEndpointCalls {
        fn from(value: GetSequencerAddressCall) -> Self {
            Self::GetSequencerAddress(value)
        }
    }

    impl ::core::convert::From<SetProductIdBookAddressCall> for IEndpointCalls {
        fn from(value: SetProductIdBookAddressCall) -> Self {
            Self::SetProductIdBookAddress(value)
        }
    }

    impl ::core::convert::From<SetSequencerAddressCall> for IEndpointCalls {
        fn from(value: SetSequencerAddressCall) -> Self {
            Self::SetSequencerAddress(value)
        }
    }

    impl ::core::convert::From<SubmitTransactionsCall> for IEndpointCalls {
        fn from(value: SubmitTransactionsCall) -> Self {
            Self::SubmitTransactions(value)
        }
    }

    ///Container type for all return fields from the `getNonce` function with signature `getNonce(address)` and selector `0x2d0335ab`
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
    pub struct GetNonceReturn(pub u64);

    ///Container type for all return fields from the `getProductOrderbook` function with signature `getProductOrderbook(uint32)` and selector `0x58833c47`
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
    pub struct GetProductOrderbookReturn(pub ::ethers::core::types::Address);

    ///Container type for all return fields from the `getSequencerAddress` function with signature `getSequencerAddress()` and selector `0x6875f178`
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
    pub struct GetSequencerAddressReturn(pub ::ethers::core::types::Address);

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
    #[derive(
        Eip712,
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
    #[eip712(
        name = "DEX",
        version = "0.1.0",
        chain_id = 1,
        verifying_contract = "0x0000000000000000000000000000000000000001"
    )]
    // debug
    pub struct Order {
        pub sender: [u8; 32],
        pub price_x18: i128,
        pub amount: i128,
        pub expiration: u64,
        pub nonce: u64,
    }
}
