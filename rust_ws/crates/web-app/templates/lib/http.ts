// @ts-ignore
import { Controller } from "stimulus"
import { debounce } from "./misc.js"

export async function post(endpoint: string, o: Object) {
	console.log("[http-controller] post", endpoint, o)
	const res = await fetch(endpoint, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(o),
	})

	return await res.json()
}

export async function get(endpoint: string, o?: Object) {
	const params = o !== undefined ? new URLSearchParams(o) : undefined
	let newEndpoint = params ? `${endpoint}?${params}` : endpoint
	const res = await fetch(newEndpoint, {
		method: "GET",
	})

	return await res.json()
}

// @ts-ignore
Stimulus.register(
	"http",
	class extends Controller {
		declare dispatch: (eventName: string, detail: any) => void

		initialize() {
			console.log("[http-controller] initialize")
			// @ts-ignore
			this.onInput = debounce(this.onInput.bind(this))
		}

		// Most of the action you click on the frontend will update directly to backend immediately.
		// Used for: combobox result item click.
		// Cross controller: if post success, we will close the combobox using its controller.
		// at parent controller have this data-action=http:selectComboboxItem->combobox#selectItemDispatched
		// @required data-action="click->http#selectItemPost"
		// @required data-http-url-param=String
		// @required data-http-payload-param={Object}
		async selectItemPost(e: Event) {
			let target = e.target as HTMLElement
			// @ts-ignore
			const params = e.params
			const { url, payload } = params
			if (!url || !payload) {
				console.error("[http-controller] url or payload is empty", params)
				return
			}
			const res = await post(url, payload)
			console.log("[http-controller] selectItemPost", res)
			if (res.success) {
				this.dispatch("selectComboboxItem", {
					detail: { targetInnerHTML: target.innerHTML },
				})
			} else {
				console.error("[http-controller] failed to post", res)
			}
		}

		async get(e: Event) {
			// @ts-ignore
			const params = e.params
			const { url, payload } = params
			if (!url) {
				console.error("[http-controller] url is empty", params)
				return
			}

			await get(url, payload)
		}

		// for input element
		// @required data-action="input->http#onInput"
		// @required data-http-url-param=String
		// @optional data-http-payload-param={Object}
		// template: <input value=
		//                  name=//@notice: in javascriptCase, it will be converted on backend to camel_case>
		// payload will be { [target.name]: target.value }
		async onInput(e: Event) {
			// @ts-ignore
			const params = e.params
			let { url, payload } = params
			if (!url || !payload) {
				console.error("[http-controller] url is empty", params)
				return
			}

			const target = e.target as HTMLInputElement
			let nameField = target.getAttribute("name")
			if (!nameField) {
				console.error("[http-controller] <input name=> field is empty", target)
				return
			}
			payload = { [nameField]: target.value, ...payload }
			await post(url, payload)
		}

		// for input element
		// @required data-action="input->http#onInput"
		// @required data-http-url-param=String
		// @optional data-http-payload-param={Object}
		// template: <button aria-checked=
		//                  name=//@notice: in javascriptCase, it will be converted on backend to camel_case>
		// payload will be { [target.name]: target.value }
		async onSwitch(e: Event) {
			// @ts-ignore
			const params = e.params
			let { url, payload } = params
			if (!url || !payload) {
				console.error("[http-controller] url is empty", params)
				return
			}

			const target = e.target as HTMLInputElement
			let isChecked = target.getAttribute("aria-checked") === "true"
			let nameField = target.getAttribute("name")
			let value = !isChecked // toggle switch value
			if (!nameField) {
				console.error("[http-controller] <input name=> field is empty", target)
				return
			}
			payload = { [nameField]: value, ...payload }
			await post(url, payload)
		}
	},
)
