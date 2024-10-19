// for example only
// @ts-ignore
import { Controller } from "stimulus"

// @ts-ignore
Stimulus.register(
	"command-block",
	class extends Controller {
		static targets = ["commandDetails"]

		declare readonly commandDetailsTarget: HTMLElement

		connect() {}

		toggleDetails() {
			// remove hidden if it's there, and add block
			if (this.commandDetailsTarget.classList.contains("hidden")) {
				this.commandDetailsTarget.classList.remove("hidden")
				this.commandDetailsTarget.classList.add("block")
			} else {
				this.commandDetailsTarget.classList.remove("block")
				this.commandDetailsTarget.classList.add("hidden")
			}
		}
	},
)
