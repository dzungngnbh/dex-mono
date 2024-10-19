// @ts-ignore
import { Controller } from "stimulus"

// utils
function getFirstPartOfUrl(url: string): string {
	return url.split("/")[3]
}

// @ts-ignore
Stimulus.register(
	"main-sidebar",
	class extends Controller {
		activeClasses = ["border", "rounded", "bg-gray-800"]

		static targets = ["dashboardLink", "portfolioLink"]

		declare readonly dashboardLinkTarget: HTMLAnchorElement
		declare readonly portfolioLinkTarget: HTMLAnchorElement

		onHistoryChange: Function | null = null

		initialize() {
			// somehow it has 2 events every url changes, I think it's htmx doing it, we get the push event
			this.selectLink(location.href)

			let self = this

			// @ts-ignore
			this.onHistoryChange = navigation.addEventListener(
				"navigate",
				(e: Event) => {
					if (e.navigationType === "push") {
						this.selectLink(e.destination.url)
					}
				},
			)
		}

		disconnect() {
			this.onHistoryChange = null
		}

		selectLink(url: string) {
			let firstPart = getFirstPartOfUrl(url)
			if (firstPart === "dashboard") {
				this.dashboardLinkTarget.classList.add(...this.activeClasses)
				this.portfolioLinkTarget.classList.remove(...this.activeClasses)
			} else if (firstPart === "portfolio") {
				this.portfolioLinkTarget.classList.add(...this.activeClasses)
				this.dashboardLinkTarget.classList.remove(...this.activeClasses)
			}
		}
	},
)
