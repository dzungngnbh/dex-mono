import { Account } from "./account"

export class AuthPage {
	#account: Account

	constructor() {
		this.#account = new Account()
	}

	async init() {
		await this.#account.init()
	}
}
