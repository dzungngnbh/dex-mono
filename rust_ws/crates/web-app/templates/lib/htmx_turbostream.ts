// @ts-ignore
import { Controller } from "stimulus"

// We use this on every turbo stream, so we can process the element
// after being rendered by turbo stream.
// For the turbo_stream component which contains htmx attributes (hx-*)
// Make sure to add the data-htmx-turbostream-target="turbostream" attribute

// @ts-ignore
Stimulus.register(
	"htmx-turbostream",
	class extends Controller {
		static targets = ["turbostream"]
		turbostreamTargetConnected(target: Element) {
			console.log("turbostreamTargetConnected")

			// @ts-ignore
			htmx.process(target)
		}
	},
)
