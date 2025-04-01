// @ts-ignore
import { Controller } from "stimulus"

// @ts-ignore
Stimulus.register(
	"dashboard-page",
	class extends Controller {
		static targets = [
			"signalServiceCommandCombobox",
			"actionServiceCommandCombobox",
		]

		static values = {
			selectedSignalServiceId: String,
			selectedActionServiceId: String,
			selectedSignalServiceCommandId: String,
			selectedActionServiceCommandId: String,
		}

		declare readonly signalServiceCommandComboboxTarget: HTMLDivElement
		declare readonly actionServiceCommandComboboxTarget: HTMLDivElement
		declare dispatch: (eventName: string, detail: any) => void

		selectedSignalServiceIdValue: number | null = null
		selectedActionServiceIdValue: number | null = null

		declare selectedSignalServiceCommandIdValue: string
		declare selectedActionServiceCommandIdValue: string

		selectSignalService(e: Event) {
			const target = e.currentTarget as HTMLElement
			// parse dataset to number
			this.selectedSignalServiceIdValue = parseInt(target.dataset.id as string)
			this.signalServiceCommandComboboxTarget.dataset.comboboxurl = `/dashboard/c/combobox_results_inner?combobox_type=service_commands&service_id=${this.selectedSignalServiceIdValue}&results_for=signalServiceCommand`

			// trigger combobox controller
			this.dispatch("selectComboboxItem", {
				detail: { targetInnerHTML: target.innerHTML },
			})
		}

		selectActionService(e: Event) {
			const target = e.currentTarget as HTMLElement
			this.selectedActionServiceIdValue = parseInt(target.dataset.id as string)
			this.actionServiceCommandComboboxTarget.dataset.comboboxurl = `/dashboard/c/combobox_results_inner?combobox_type=service_commands&service_id=${this.selectedActionServiceIdValue}&results_for=actionServiceCommand`

			this.dispatch("selectComboboxItem", {
				detail: { targetInnerHTML: target.innerHTML },
			})
		}

		selectSignalServiceCommand(e: Event) {
			if (this.selectedSignalServiceIdValue === null) {
				return
			}

			const target = e.currentTarget as HTMLElement
			this.selectedSignalServiceCommandIdValue = target.dataset.id as string

			this.dispatch("selectComboboxItem", {
				detail: { targetInnerHTML: target.innerHTML },
			})
		}

		// values changed
		selectedSignalServiceIdValueChanged() {
			this.selectedSignalServiceCommandIdValue = ""
		}

		selectedActionServiceIdValueChanged() {
			this.selectedActionServiceCommandIdValue = ""
		}

		selectedSignalServiceCommandIdValueChanged() {
			if (this.selectedSignalServiceCommandIdValue === "") {
				let title = this.signalServiceCommandComboboxTarget.querySelector(
					"[data-comboboxtitle]",
				)
				title!.innerHTML = "Select signal event"
			}
		}

		selectedActionServiceCommandIdValueChanged() {
			if (this.selectedActionServiceCommandIdValue === "") {
				let title = this.signalServiceCommandComboboxTarget.querySelector(
					"[data-comboboxtitle]",
				)
				title!.innerHTML = "Select action event"
			}
		}

		selectActionServiceCommand(e: Event) {
			if (this.selectedActionServiceIdValue == null) {
				return
			}

			const target = e.currentTarget as HTMLElement
			this.selectedActionServiceCommandIdValue = target.dataset.id as string
			this.dispatch("selectComboboxItem", {
				detail: { targetInnerHTML: target.innerHTML },
			})
		}

		async createStrategy() {
			// check if all selected fields is not empty and create new strategy
			if (
				this.selectedSignalServiceCommandIdValue.length === 0 ||
				this.selectedActionServiceCommandIdValue.length === 0
			) {
				console.info("[dashboard-page] some fields are empty")
				return
			}

			const res = await fetch("/api/strategy/create", {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					signal_service_command_id: this.selectedSignalServiceCommandIdValue,
					action_service_command_id: this.selectedActionServiceCommandIdValue,
				}),
			})

			if (res.status !== 200) {
				console.error(
					"[dashboard-page] failed to create strategy",
					await res.json(),
				)
				return
			}

			const { data } = await res.json()
			const { id } = data

			// replace this with your own http
			// htmx
			// 	.ajax("GET", `${editorPageComponentRouteHtmx(id)}/page`, {
			// 		target: "#mainContent",
			// 		swap: "outerHTML",
			// 	})
			// 	.then(() => {
			// 		window.history.pushState(
			// 			window.history.state,
			// 			"",
			// 			`/dashboard/editor/${data.id}`,
			// 		)
			// 	})
		}
	},
)
