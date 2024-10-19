// @ts-ignore
import { Controller } from "stimulus"

// @ts-ignore
Stimulus.register(
	"strategy-monitor-page",
	class extends Controller {
		// @ts-ignore
		chart: null

		initialize() {
			this.initHighcharts()
		}

		initHighcharts() {
			// @ts-ignore
			this.chart = new Highcharts.Chart("chart", {
				credits: {
					enabled: false,
				},
				chart: {
					type: "column",
					backgroundColor: "transparent",
					style: {
						fontFamily: "Inter",
					},
					animation: false,
				},

				data: {
					// TODO: change later
					rowsURL:
						"https://localhost:3000/dashboard/strategy/175d0b92-0573-40c0-abb9-94b1c424feb8/c/highchart_data.json",
					firstRowAsNames: false,
					enablePolling: true,
					dataRefreshRate: 5,
					parsed: function (columns) {
						// console.log('Parsed', columns);
						const chart = this.chart

						chart.update({
							series: [
								{
									name: "Success",
									data: columns[0],
									color: "#94D2BD",
								},
								{
									name: "Unmet condition",
									data: columns[1],
									color: "#E9D8A6",
								},
								{
									name: "Failed",
									data: columns[2],
									color: "#AE2012",
								},
							],
						})

						return false
					},
				},

				title: {
					text: "Recent 30 Days",
					align: "left",
					style: {
						color: "#ffffff",
					},
				},
				xAxis: {
					categories: [
						"30d ago",
						"29d ago",
						"28d ago",
						"27d ago",
						"26d ago",
						"25d ago",
						"24d ago",
						"23d ago",
						"22d ago",
						"21d ago",
						"20d ago",
						"19d ago",
						"18d ago",
						"17d ago",
						"16d ago",
						"15d ago",
						"14d ago",
						"13d ago",
						"12d ago",
						"11d ago",
						"10d ago",
						"9d ago",
						"8d ago",
						"7d ago",
						"6d ago",
						"5d ago",
						"4d ago",
						"3d ago",
						"2d ago",
						"1d ago",
						"Today",
					],
					labels: {
						style: {
							color: "#ffffff",
						},
					},
				},
				yAxis: {
					min: 0,
					title: {
						text: "Number of runs",
						style: {
							color: "#ffffff",
						},
					},
					stackLabels: {
						enabled: true,
					},
					gridLineDashStyle: "dash",
					gridLineColor: "#64748b",
					labels: {
						style: {
							color: "#ffffff",
						},
					},
				},
				legend: {
					align: "center",
					verticalAlign: "bottom",
					x: 0,
					y: 0,
					itemStyle: {
						color: "#ffffff",
					},
				},
				tooltip: {
					headerFormat: "<b>{point.x}</b><br/>",
					pointFormat: "{series.name}: {point.y}<br/>Total: {point.stackTotal}",
				},
				plotOptions: {
					column: {
						stacking: "normal",
						dataLabels: {
							enabled: true,
						},
					},
				},
				series: [
					{
						name: "Success",
						// data: randomSuccess,
						color: "#94D2BD",
					},
					{
						name: "Unmet condition",
						// data: randomUnmetConditions,
						color: "#E9D8A6",
					},
					{
						name: "Failed",
						// data: randomFail,
						color: "#AE2012",
					},
				],
			})
		}
	},
)
