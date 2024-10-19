// @ts-ignore
import { Controller } from "stimulus"

// @ts-ignore
Stimulus.register(
	"editor-page",
	class extends Controller {
		static targets = ["commandBlock"]
		declare readonly commandBlockTargets: HTMLElement[]

		highlightClass = "border-slate-400"
		selectedCommandBlock: HTMLElement | null = null

		selectCommandBlock(e: Event) {
			let currentTarget = e.currentTarget as HTMLElement
			if (this.selectedCommandBlock === currentTarget) {
				return
			}

			this.selectedCommandBlock = currentTarget
			this.commandBlockTargets.forEach((commandBlock) => {
				commandBlock.classList.remove(this.highlightClass)
			})

			currentTarget.classList.add(this.highlightClass)
		}
	},
)
