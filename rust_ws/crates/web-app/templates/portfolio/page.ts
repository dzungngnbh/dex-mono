// @ts-ignore
import { Controller } from "stimulus"

// @ts-ignore
Stimulus.register(
	"portfolio-page",
	class extends Controller {
		initialize() {
			//       const columnDefs = [
			//         {headerName: "Name", field: "make"},
			//         {headerName: "Amount", field: "model"},
			//         {headerName: "Price", field: "price"},
			//         {headerName: "Total", field: "total"}
			//       ];
			//
			// // specify the data
			//       const rowData = [
			//         {make: "ETH", amount: 1, price: 1800, total: 1800},
			//       ];
			//
			// // let the grid know which columns and what data to use
			//       const gridOptions = {
			//         columnDefs: columnDefs,
			//         rowData: rowData,
			//         rowSelection: 'single',
			//       };
			//
			// // setup the grid after the page has finished loading
			//       const gridDiv = document.querySelector('#portfolio-table');
			//       // @ts-ignore
			//       new agGrid.Grid(gridDiv, gridOptions);
		}
	},
)
