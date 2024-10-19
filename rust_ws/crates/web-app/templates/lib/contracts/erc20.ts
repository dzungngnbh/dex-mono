import { EndpointAddress } from "./constants.js"
import { erc20Abi } from "./abis/IERC20.js"

// turn this to functional functions
export async function approveEndpoint(
	walletClient,
	address: string,
	amount: bigint,
): Promise<any> {
	return await walletClient.writeContract({
		address,
		abi: erc20Abi,
		functionName: "approve",
		args: [EndpointAddress, amount],
	})
}
