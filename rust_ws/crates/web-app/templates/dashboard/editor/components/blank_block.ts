// @ts-ignore
import { Controller } from "stimulus"

// @ts-ignore
Stimulus.register(
	"blank-block",
	class extends Controller {
		static targets = ["deleteButton"]

		declare readonly deleteButtonTarget: HTMLElement

		// initialize() {
		//   console.log("initialize editor")
		// }

		connect() {}

		showDeleteButton() {
			this.deleteButtonTarget.classList.add("inline-flex")
			this.deleteButtonTarget.classList.remove("hidden")
		}

		hideDeleteButton() {
			this.deleteButtonTarget.classList.add("hidden")
			this.deleteButtonTarget.classList.remove("inline-flex")
		}
	},
)
