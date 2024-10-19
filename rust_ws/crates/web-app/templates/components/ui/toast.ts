// @ts-ignore
import { Controller } from "stimulus"

// @ts-ignore
Stimulus.register(
	"toast",
	class extends Controller {
		static targets = ["toastList", "toastItem"]
		autoHideTimout = 10000

		declare toastListTarget: HTMLDivElement
		declare toastItemTarget: HTMLDivElement

		async addToast(e: Event) {
			const params = {
				toast_title: e.params?.title,
				toast_description: e.params?.description,
			}

			const queryParams = new URLSearchParams(params).toString()
			let res = await fetch(`/c/toast_item?${queryParams}`)
			if (!res.ok) {
				return
			}
			let html = await res.text()

			this.toastListTarget.insertAdjacentHTML("beforeend", html)
			let newToast = this.toastListTarget.lastElementChild as HTMLDivElement

			setTimeout(() => {
				newToast.remove()
			}, this.autoHideTimout)
		}

		removeToast(e: Event) {
			let toast = (e.target as HTMLButtonElement).parentElement?.parentElement
			toast?.remove()
		}
	},
)
