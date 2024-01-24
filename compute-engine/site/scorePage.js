




// const backPageButton = document.getElementById("back-page");

// backPageButton.textContent = "Back Page";

// backPageButton.addEventListener("click", async () => {
//   location.href = 'index.html';
// });


// const table = document.createElement("table");
// const tableBody = document.createElement("tbody");

// generateTable()


// function getTableValues() {
//   return fetch("http://localhost:5057/", { method: 'GET' })
//     .then((response) => response.json())
//     .then((data) => fillOutTable(data));
// }

// function generateTable() {
//   tableValues = getTableValues();
// }

// function prettifyText(text) {
//   text = text.replace("_", " ");
//   text = text[0].toUpperCase() + text.slice(1);
//   return text;
// }

// function fillOutTable(data) {

//   columnNames = Object.keys(data[0])
//   // Clears table
//   while (tableBody.firstChild) {
//     tableBody.removeChild(tableBody.lastChild);
//   }
//   const columnRow = document.createElement("tr");


//   // Add column names
//   columnNames.forEach((element) => {
//     const cell = document.createElement("th");
//     const cellText = document.createTextNode(prettifyText(element));
//     cell.appendChild(cellText)
//     columnRow.appendChild(cell);
//   });
//   tableBody.appendChild(columnRow);


//   // Updates table with new data
//   for (let ii = 0; ii < data.length; ii++) {
//     const row = document.createElement("tr");

//     columnNames.forEach((element) => {
//       const cell = document.createElement("td");
//       const cellText = document.createTextNode(data[ii][element]);
//       cell.appendChild(cellText);
//       row.appendChild(cell);
//     });
//     tableBody.appendChild(row);
//   }
//   table.appendChild(tableBody);
//   document.body.appendChild(table);
//   table.setAttribute("border", "2");
// }


// function addNewElement() {
//   alert("Not yet implemented");
// }
