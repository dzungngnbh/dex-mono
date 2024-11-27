// @ts-ignore
import { Controller } from "stimulus"

import { Account } from "../lib/account.js"

// @ts-ignore
Stimulus.register(
	"trade-page",
	class extends Controller {
		static values = { productId: Number, sizeIncrement: Number }
		static targets = [
			"assetAmount",
			"quoteAmount",
			"actionButton", // buy or sell
			"price",
			"marketListButton",
			"marketList"
		]

		// di
		#account: Account

		// localState
		#buyOrSell: 0 // 0 is buy, 1 is sell
		#asssetAmount: number
		#isMarketListVisible: boolean = false

		initialize() {
			this.#account = new Account()
			this.#account.init()
			this.#setupMarketListClickOutside()
		}

		#setupMarketListClickOutside() {
			document.addEventListener('click', (event) => {
				const marketListButton = this.marketListButtonTarget
				const marketList = this.marketListTarget

				if (!marketListButton.contains(event.target as Node) &&
					!marketList.contains(event.target as Node) &&
					this.#isMarketListVisible) {
					this.#isMarketListVisible = false
					this.#updateMarketListVisibility()
				}
			})
		}

		#updateMarketListVisibility() {
			console.log("TEST")
			if (this.#isMarketListVisible) {
				this.marketListTarget.classList.remove('hidden')
				this.marketListTarget.classList.add('block')
				this.marketListButtonTarget.setAttribute('aria-expanded', 'true')
			} else {
				this.marketListTarget.classList.remove('block')
				this.marketListTarget.classList.add('hidden')
				this.marketListButtonTarget.setAttribute('aria-expanded', 'false')
			}
		}

		// account
		depositCollateral({ params: { productId } }) {
			if (!this.#account.isConnected()) {
				return
			}

			this.#account.depositCollateral(productId, 1)
		}

		placeOrder(e: Event) {
			e.preventDefault()
			if (!this.#account.isConnected()) {
				return
			}

			const amount = this.#buyOrSell === 0 ? this.#asssetAmount : -this.#asssetAmount
		}

		// ui
		setAssetAmount(e: Event) {
			const amount: number = (e.target as HTMLInputElement).value
			const price: number = Number(this.priceTarget.dataset.value)
			const totalQuote = amount * price

			// update ui
			this.quoteAmountTarget.value = totalQuote

			if (amount <= this.sizeIncrementValue) {
				// update ui
				this.#updateActionButton(false)
				return
			}

			this.#asssetAmount = amount
			this.#updateActionButton(true)
		}

		toggleMarketList(e: Event) {
			this.#isMarketListVisible = !this.#isMarketListVisible
			this.#updateMarketListVisibility()
		}

		setBuyOrSell({ params: { buyOrSell } }) {
			this.#buyOrSell = buyOrSell
		}

		#actionButtonDisableClasses = ["cursor-not-allowed"]
		#actionButtonActiveClasses = ["cursor-pointer"]
		#updateActionButton(isActive: boolean) {
			if (isActive) {
				this.actionButtonTarget.classList.remove(...this.#actionButtonDisableClasses)
				this.actionButtonTarget.classList.add(...this.#actionButtonActiveClasses)
				this.actionButtonTarget.removeAttribute("disabled")
			} else {
				this.actionButtonTarget.classList.remove(...this.#actionButtonActiveClasses)
				this.actionButtonTarget.classList.add(...this.#actionButtonDisableClasses)
				this.actionButtonTarget.setAttribute("disabled", "")
			}
		}

		disconnect() {
			// Clean up event listeners when controller is disconnected
			document.removeEventListener('click', this.#setupMarketListClickOutside)
		}
	},
)
