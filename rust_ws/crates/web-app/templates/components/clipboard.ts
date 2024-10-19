// @ts-ignore
import { Controller } from "stimulus"

// @ts-ignore
Stimulus.register(
	"clipboard",
	class extends Controller {
		static targets = ["source"]

		declare readonly sourceTarget: HTMLElement

		// initialize() {
		//   console.log("clipboard connected")
		// }
		//
		copy() {
			// check if sourceTarget is an input or textarea
			if (
				this.sourceTarget instanceof HTMLInputElement ||
				this.sourceTarget instanceof HTMLTextAreaElement
			) {
				navigator.clipboard.writeText(this.sourceTarget.value)
				return
			}

			if (this.sourceTarget instanceof HTMLElement) {
				let val = this.sourceTarget.dataset.clipboardvalue
				if (val) {
					navigator.clipboard.writeText(val)
					return
				}
			}
		}
	},
)
