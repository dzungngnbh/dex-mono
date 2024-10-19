// signal for everyone to sub to
import {
	EthereumClient,
	w3mConnectors,
	w3mProvider,
	WagmiCore,

} from "https://unpkg.com/@web3modal/ethereum"
const { configureChains, createConfig, getAccount, watchAccount } = WagmiCore
import { Web3Modal } from "https://unpkg.com/@web3modal/html"

import {
	createWalletClient,
	custom,
	parseEther,
	toHex,
} from "viem"
import { signMessage } from "viem/accounts/index.js"
import "viem/window/index.js"
import { localMainnet } from "./contracts/chains.js"
import * as clientLib from "./contracts/client.js"
import * as utilsLib from "./contracts/utils.js"
import * as httpLib from "./http.js"

if (typeof window !== "undefined") {
	if (!window.global) {
		window.global = window
	}
	if (!window.process) {
		// @ts-ignore
		window.process = { env: {} }
	}
}

// contracts
import { EndpointContract } from "./contracts/endpoint.js"
import { approveEndpoint } from "./contracts/erc20.js"
import { ProductAddresses } from "./contracts/constants.js"
import { getCookies } from "./misc.js"

// import { mainnet } from "viem/chains/index.js"
// const chains = [mainnet]
const projectId = "ad81210d59dff743ef3ad036e9feb6f7"

enum AccountStatus {
	NOT_CONNECTED = 0,
	CONNECTED,
	NOT_FOUND,
	UNKNOWN,
}

const SIGN_MESSAGE = "By joining, you agree to our Terms of Service ( https://tradingexec.xyz/legal/terms ) and Privacy Policy ( https://tradingexec.xyz/legal/policy ) ."

class Account {
	private #endpointContract: any
	private #accountStatus: AccountStatus
	private #currentAccount: any
	private _watchAccount: any

	private #walletClient: any = null
	private #publicClient: any = null

	#defaultSubaccount: string

	// TODO: init on certain chain only
	async init() {
		this.#defaultSubaccount = this.#constructSubaccount("default")
		await this.initModal()
	}

	public isConnected() {
		return this.#accountStatus === AccountStatus.CONNECTED
	}

	private async initModal() {
		const chains = [localMainnet]
		const { publicClient } = configureChains(chains, [
			w3mProvider({ projectId }),
		])
		const wagmiConfig = createConfig({
			autoConnect: true,
			connectors: w3mConnectors({ projectId, chains }),
			publicClient,
		})
		const ethereumClient = new EthereumClient(wagmiConfig, chains)
		new Web3Modal({ projectId }, ethereumClient)

		this.#publicClient = clientLib.publicClient
		this._watchAccount = watchAccount(async () => {
			this.#currentAccount = await getAccount()
			this.#setAccountStatus()
			console.log("currentAccount", this.#currentAccount)

			if (this.#accountStatus === AccountStatus.NOT_CONNECTED) {
				this.#walletClient = null
			} else if (
				this.#accountStatus === AccountStatus.CONNECTED &&
				!this.#walletClient
			) {
				const [account] = await window.ethereum!.request({ method: 'eth_requestAccounts' })
				this.#walletClient = createWalletClient({
					account,
					chain: localMainnet,
					transport: custom(window.ethereum!),
				})

				const isLogin = await this.#isUserLogin()
				if (!isLogin) {
					const res = await this.#login()
					if (!res) {
						console.log("login failed")
						return
					}
				}

				this.#endpointContract = new EndpointContract(
					this.#publicClient,
					this.#walletClient,
				)

				const res = await this.#endpointContract.getNonce(this.#currentAccount.address)
				console.log("getNonce test", res)
			}
		})
	}

	async #login(): Promise<boolean> {
		// sign message
		const signedMessage = await this.#walletClient.signMessage({ message: SIGN_MESSAGE })
		const sender = this.#currentAccount.address
		const res = await httpLib.post("/api/auth/login", {
			sender,
			signedMessage,
		})
		const { data, success } = res
		if (!success) {
			console.log("login failed", res)
			return false
		}
		return true
	}

	// TODO: Make req to server
	async #isUserLogin() : Promise<boolean> {
		const res = await httpLib.get("/api/auth/is_login")
		const { data, success } = res
		if (success && data.is_login === true) {
			return true
		}
		 return false
	}

	#setAccountStatus() {
		if (!this.#currentAccount.isConnected) {
			this.#accountStatus = AccountStatus.NOT_CONNECTED
			return
		}

		this.#accountStatus = AccountStatus.CONNECTED
	}

	// return hex of subaccount
	#constructSubaccount(sub: string): string {
		if (this.#accountStatus !== AccountStatus.CONNECTED) {
			return ""
		}

		return toHex(sub, {size: 12})
	}

	// actions
	public async approve(address: string, amount: bigint) {
		if (this.#accountStatus !== AccountStatus.CONNECTED) {
			return
		}

		return await approveEndpoint(this.#walletClient, address, amount)
	}

	// approve and deposit
	public async depositCollateral(productId: number, amount: number) {
		if (this.#accountStatus !== AccountStatus.CONNECTED) {
			return
		}

		const amountEther = parseEther(amount.toString())
		const tokenAddress = ProductAddresses[productId]
		const receipt = await this.approve(tokenAddress, amountEther)
		const [tx, isSuccess] = await utilsLib.isSuccess(this.#publicClient, receipt)
		if (!isSuccess) {
			console.log("Approve failed", tx)
			return
		} 
		console.log("success approve", tx)

		console.log("defaultSubaccount", this.#defaultSubaccount)
		const subaccount = toHex("default", {size: 12})
		await this.#endpointContract.depositCollateral(subaccount, productId, amountEther)
	}
}

export { Account } 