// @ts-ignore
import { debounce } from "../../../lib/misc.js"
import { Controller } from "stimulus"

// TODO: import from unpkg

// @ts-ignore
Stimulus.register(
	"combobox",
	class extends Controller {
		static targets = [
			"combobox",
			"comboboxResults",
			"comboboxResultsItems",
			"comboboxResultsWrapper",
			"resultItem",
			"searchInput",
			"triggerElement",
		]

		searchIndex = new FlexSearch.Worker({
			tokenize: "forward",
			optimize: true,
			resolution: 1,
			minlength: 3,
		})

		declare readonly comboboxResultsTarget: HTMLDivElement
		declare readonly comboboxResultsItemsTarget: HTMLDivElement
		declare readonly comboboxResultsWrapperTarget: HTMLDivElement
		declare readonly resultItemTargets: HTMLElement[]
		declare readonly searchInputTarget: HTMLInputElement
		declare readonly triggerElementTargets: HTMLElement[]

		// TODO: use another attribute like disable, can only be enabled when one of them changed

		currentComboboxLabel = "" // in button data-comboboxLabel
		currentClickedCombobox: HTMLElement | null = null
		currentComboboxResultsWrapperWidthClass: string | null = null

		initialize() {
			this.search = debounce(this.search.bind(this))
		}

		/// Used for: combobox trigger click from other controller,
		/// it swaps the html and close the combobox results.
		selectItemDispatched({ detail: { targetInnerHTML } }: CustomEvent) {
			console.log("[combobox-controller] selectItemDispatched", targetInnerHTML)
			this.selectItem_(targetInnerHTML)
		}

		selectItem(e: Event) {
			this.selectItem_(e.currentTarget!.innerHTML)
		}

		// internal function
		// TODO: refactor to _selectItem
		selectItem_(targetInnerHTML: string) {
			if (!targetInnerHTML || targetInnerHTML.length === 0) {
				console.error("[combobox-controller] targetInnerHTML is empty")
				return
			}

			// replace the content of currentClickedCombobox
			this.currentClickedCombobox!.querySelector(
				"[data-comboboxtitle]",
			)!.innerHTML = targetInnerHTML
			this.closeResults()
		}

		async fetchAndSwap(url: string) {
			const res = await fetch(url)
			const html = await res.text()
			this.comboboxResultsTarget.innerHTML = html

			// add data to index
			for (const item of this.resultItemTargets) {
				const id = item.dataset.id
				const text = item.dataset.value
				await this.searchIndex.addAsync(id, text)
			}
		}

		async showResults(e: Event) {
			let currentTarget = e.currentTarget as HTMLElement
			if (this.currentClickedCombobox === currentTarget) return // if click on the same element, do nothing

			const url = currentTarget.dataset.comboboxurl
			if (!url) {
				console.error(
					"[combobox-controller] data-comboboxurl not found on clicked combobox element",
				)
				return
			}
			await this.fetchAndSwap(url)

			if (this.comboboxResultsTarget.classList.contains("hidden")) {
				this.currentComboboxLabel = currentTarget.innerText

				const boundingRect = currentTarget.getBoundingClientRect()

				const offsetX = boundingRect.left
				const offsetY = boundingRect.bottom + 6
				const width = currentTarget.clientWidth
				this.currentComboboxResultsWrapperWidthClass = `w-[${width}px]`

				this.comboboxResultsTarget.style.transform = `translate(${offsetX}px, ${offsetY}px)`
				this.comboboxResultsWrapperTarget.classList.add(
					this.currentComboboxResultsWrapperWidthClass,
				)
				this.currentClickedCombobox = currentTarget
				this.comboboxResultsTarget.classList.remove("hidden")
				this.searchInputTarget.focus()

				// Use an arrow function for the event listener
				window.addEventListener("click", this.windowClickOutside)
				window.addEventListener("keydown", this.closeOnEscape)
			}
		}

		/// Search with value is query string
		/// if we have data-combobox-remote="true" in input of combobox
		/// then we will fetch the data from server
		search(e: Event) {
			let target = e.target as HTMLInputElement
			let value = target.value

			// show all results if search string is empty
			if (value.length < 3) {
				for (const resultItem of this.resultItemTargets) {
					if (resultItem.classList.contains("hidden")) {
						resultItem.classList.remove("hidden")
					}
				}
				return
			}

			let remoteUrl = target.dataset.comboboxRemoteUrl
			if (remoteUrl) {
				let res = fetch(`${remoteUrl}?q=${value}`).then((res) => {
					if (res.status === 200) {
						res.text().then((html) => {
							this.comboboxResultsItemsTarget.outerHTML = html
						})
					} else {
						console.error(
							"[combobox-controller] failed to fetch data from server",
						)
					}
				})
			} else {
				// search locally
				let res = this.searchIndex
					.searchAsync(value)
					.then((results: string[]) => {
						// TODO: if results is empty show no results found

						for (const resultItem of this.resultItemTargets) {
							if (results.includes(resultItem.dataset.id!)) {
								resultItem.classList.remove("hidden")
							} else {
								resultItem.classList.add("hidden")
							}
						}
					})
			}
		}

		closeResults() {
			this.comboboxResultsTarget.classList.add("hidden")
			this.comboboxResultsWrapperTarget.classList.remove(
				this.currentComboboxResultsWrapperWidthClass!,
			)

			// Remove the event listener using the same function reference
			window.removeEventListener("click", this.windowClickOutside)
			window.removeEventListener("keydown", this.closeOnEscape)
			this.currentClickedCombobox = null
		}

		windowClickOutside = (e: Event) => {
			let target = e.target as HTMLElement

			if (
				this.currentClickedCombobox &&
				this.currentClickedCombobox.contains(target)
			)
				return // if click on the same element, do nothing

			// if click on another trigger element
			if (this.triggerElementTargets.includes(target)) {
				this.closeResults()
				// this.showResults(e) // dont need to do this, since it will trigger the showResults anyway
				return
			}

			// if click outside of combobox results
			if (
				!this.comboboxResultsTarget.contains(target) &&
				this.comboboxResultsTarget !== target
			) {
				this.closeResults()
			}
		}

		closeOnEscape = (e: KeyboardEvent) => {
			if (e.key === "Escape") {
				this.closeResults()
			}
		}
	},
)
