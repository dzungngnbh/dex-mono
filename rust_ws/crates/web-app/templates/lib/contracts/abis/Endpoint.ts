export const endpointAbi = [
	{
		type: "function",
		name: "depositCollateral",
		inputs: [
			{
				name: "subaccountName",
				type: "bytes12",
				internalType: "bytes12",
			},
			{
				name: "productId",
				type: "uint32",
				internalType: "uint32",
			},
			{
				name: "amount",
				type: "uint128",
				internalType: "uint128",
			},
		],
		outputs: [],
		stateMutability: "nonpayable",
	},
	{
		type: "function",
		name: "getNonce",
		inputs: [
			{
				name: "_sender",
				type: "address",
				internalType: "address",
			},
		],
		outputs: [
			{
				name: "",
				type: "uint64",
				internalType: "uint64",
			},
		],
		stateMutability: "view",
	},
	{
		type: "function",
		name: "getOracleTime",
		inputs: [],
		outputs: [
			{
				name: "",
				type: "uint128",
				internalType: "uint128",
			},
		],
		stateMutability: "view",
	},
	{
		type: "function",
		name: "getProductOrderbook",
		inputs: [
			{
				name: "_productId",
				type: "uint32",
				internalType: "uint32",
			},
		],
		outputs: [
			{
				name: "",
				type: "address",
				internalType: "contract IOffchainOrderbook",
			},
		],
		stateMutability: "view",
	},
	{
		type: "function",
		name: "getSequencerAddress",
		inputs: [],
		outputs: [
			{
				name: "",
				type: "address",
				internalType: "address",
			},
		],
		stateMutability: "view",
	},
	{
		type: "function",
		name: "setProductIdBookAddress",
		inputs: [
			{
				name: "_productId",
				type: "uint32",
				internalType: "uint32",
			},
			{
				name: "_bookAddress",
				type: "address",
				internalType: "address",
			},
		],
		outputs: [],
		stateMutability: "nonpayable",
	},
	{
		type: "function",
		name: "setSequencerAddress",
		inputs: [
			{
				name: "_sequencerAddress",
				type: "address",
				internalType: "address",
			},
		],
		outputs: [],
		stateMutability: "nonpayable",
	},
	{
		type: "function",
		name: "submitTransactions",
		inputs: [
			{
				name: "txs",
				type: "bytes[]",
				internalType: "bytes[]",
			},
		],
		outputs: [],
		stateMutability: "nonpayable",
	},
	{
		type: "event",
		name: "SubmitTransactions",
		inputs: [],
		anonymous: false,
	},
]
