import { numberToBytes, toBytes, toHex, parseEther } from "viem"

import { EndpointAddress } from "./constants.js"
import { endpointAbi } from "./abis/Endpoint.js"

export class EndpointContract {
	#publicClient: any
	#walletClient: any

	constructor(publicClient, walletClient) {
		this.#publicClient = publicClient
		this.#walletClient = walletClient
	}

	public async getNonce(address: string): Promise<any> {
		return await this.#publicClient.readContract({
			address: EndpointAddress,
			abi: endpointAbi,
			functionName: "getNonce",
			args: [address],
		})
	}

	public async depositCollateral(
		subaccount: string, // hex value from toHex
		productId: number,
		amount: bigint, // uint128
	): Promise<any> {
		console.log("subaccount", subaccount)
		const { request } = await this.#walletClient.writeContract({
			address: EndpointAddress,
			abi: endpointAbi,
			functionName: "depositCollateral",
			args: [subaccount, productId, amount],
		})
		return await this.#walletClient.writeContract(request)
	}
}
