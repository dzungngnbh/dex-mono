import { get as httpGet, post as httpPost } from "../../../lib/http.js"
// @ts-ignore
import { Controller } from "stimulus"

function contains(elements: HTMLElement[], target: HTMLElement) {
	return elements.some((element) => element.contains(target))
}

// @ts-ignore
Stimulus.register(
	"editor",
	class extends Controller {
		static targets = ["blockOptionMenu", "blankBlock", "addBlankBlockButton"]

		declare readonly blockOptionMenuTarget: HTMLElement
		declare readonly blankBlockTarget: HTMLElement
		declare readonly blankBlockTargets: HTMLElement[]
		declare readonly addBlankBlockButtonTarget: HTMLElement
		declare readonly addBlankBlockButtonTargets: HTMLElement[]

		static values = {
			strategyId: String,
		}
		declare strategyIdValue: string

		// use to track if blockOptionMenu is open and block option is being reclicked
		currentClickedBlankBlock: HTMLElement | null = null

		// Show block option for blank block click
		// This is a version of combobox, we have a template hiding somewhere, and show it
		// latest version
		// if the menu is open, just move it position
		showBlockOptionMenu(e: Event) {
			let currentTarget = e.currentTarget as HTMLElement

			const boundingRect = currentTarget.getBoundingClientRect()

			const offsetX = boundingRect.left
			const offsetY = boundingRect.bottom + 6

			this.blockOptionMenuTarget.style.transform = `translate(${offsetX}px, ${offsetY}px)`
			// this.comboboxResultsWrapperTarget.classList.add(
			//   this.currentComboboxResultsWrapperWidthClass,
			// )
			this.currentClickedBlankBlock = currentTarget
			if (this.blockOptionMenuTarget.classList.contains("hidden")) {
				this.blockOptionMenuTarget.classList.remove("hidden")
			}

			// Use an arrow function for the event listener
			window.addEventListener("click", this.windowClickOutside)
			window.addEventListener("keydown", this.closeBlockMenuOptionOnEscape)
		}

		closeBlockOptionMenu() {
			this.blockOptionMenuTarget.classList.add("hidden")

			// Remove the event listener using the same function reference
			window.removeEventListener("click", this.windowClickOutside)
			window.removeEventListener("keydown", this.closeBlockMenuOptionOnEscape)
			this.currentClickedBlankBlock = null
		}

		closeBlockMenuOptionOnEscape = (e: KeyboardEvent) => {
			if (e.key === "Escape") {
				this.closeBlockOptionMenu()
			}
		}

		windowClickOutside = (e: Event) => {
			let target = e.target as HTMLElement

			// if click on the same element, do nothing
			if (
				this.currentClickedBlankBlock &&
				this.currentClickedBlankBlock.contains(target)
			)
				return

			// if menu is shown, and click on menu -> do nothing
			if (
				!this.blockOptionMenuTarget.classList.contains("hidden") &&
				this.blockOptionMenuTarget.contains(target)
			)
				return

			if (
				this.currentClickedBlankBlock && // menu is open
				this.blankBlockTargets.some((e) => e.contains(target)) // another target
			) {
				this.showBlockOptionMenu(e) // move position to new target
				return
			}

			// click outside of menu, close menu
			this.closeBlockOptionMenu()
		}
		// End block_option_menu

		// Menu options
		async replaceBlankBlock(e: Event) {
			// @ts-ignore
			const params = e.params
			const { serviceCommandId } = params

			let strategyCommandId =
				this.currentClickedBlankBlock!.dataset.strategyCommandId
			try {
				const res = await httpPost(
					`/api/editor/${this.strategyIdValue}/replace_blank_block`,
					{
						strategyCommandId,
						serviceCommandId,
						strategyId: this.strategyIdValue,
					},
				)
				const { data, success } = res
				if (!success) {
					console.error("error")
					return
				}

				// replace currentClickedBlankBlock with the new html in data
				this.currentClickedBlankBlock!.outerHTML = data
			} catch (e) {
				console.error(e)
			}

			// replace currentClickedBlankBlock with the new html in data
			// this.currentClickedBlankBlock!.outerHTML = data

			// console.log(params)
		}
		//

		async addBlankBlock(e: Event) {
			// @ts-ignore
			const params = e.params
			const { blockOrder } = params
			if (blockOrder === null || blockOrder === undefined) {
				console.error("data-editor-block-order-param is not defined")
				return
			}

			let currentTarget = e.currentTarget as HTMLElement

			try {
				const res = await httpPost(
					`/api/editor/${this.strategyIdValue}/add_blank_block`,
					{
						strategyId: this.strategyIdValue,
						blockOrder,
					},
				)
				const { data, success } = res
				if (!success) {
					console.error("error")
					return
				}

				// increase data-editor-block-order-param of all blank block after this block
				// loop through addBlankBlockButtonTargets
				for (let i = 0; i < this.addBlankBlockButtonTargets.length; i++) {
					let addBlankBlockButton = this.addBlankBlockButtonTargets[i]
					let editorBlockOrderParam =
						addBlankBlockButton.dataset!.editorBlockOrderParam
					if (!editorBlockOrderParam) continue
					let addBlankBlockButtonBlockOrder = parseInt(editorBlockOrderParam)
					if (addBlankBlockButtonBlockOrder < blockOrder) continue
					addBlankBlockButton.setAttribute(
						"data-editor-block-order-param",
						`${addBlankBlockButtonBlockOrder + 1}`,
					)
				}

				// replace addBlankBlockButton with the new html in data ( which contains new blank block + command_connector )
				currentTarget.outerHTML = data
			} catch (e) {
				console.error(e)
			}
		}
	},
)
