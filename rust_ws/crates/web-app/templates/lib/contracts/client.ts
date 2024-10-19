import { localMainnet } from "./chains.js"
import { createPublicClient, http } from "viem"

export const publicClient = createPublicClient({
	chain: localMainnet,
	transport: http(),
})
