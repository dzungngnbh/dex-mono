import { defineChain } from "viem"

export const localMainnet = defineChain({
	id: 31337,
	name: "LMainnet",
	nativeCurrency: {
		decimals: 18,
		name: "Ether",
		symbol: "tETH",
	},
	rpcUrls: {
		default: {
			http: ["http://127.0.0.1:8545"],
		},
	},
	blockExplorers: {},
	contracts: {},
})
