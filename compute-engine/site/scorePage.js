




const backPageButton = document.getElementById("back-page");

backPageButton.textContent = "Back Page";


function getTableValues() {
  return fetch("http://localhost:5057/", { method: 'GET' })
    .then((response) => response.json())
    .then((data) => fillOutTable(data));
}

function generateTable() {
  tableValues = getTableValues();
}

function fillOutTable(data) {
  const table = document.createElement("table");
  const tableBody = document.createElement("tbody");


  for (let ii = 0; ii < data.length; ii++) {
    const row = document.createElement("tr");

    for (let jj = 0; jj < data[ii].length; jj++) {
      const cell = document.createElement("td");
      const cellText = document.createTextNode(data[ii][jj]);
      cell.appendChild(cellText);
      row.appendChild(cell);
    }
    tableBody.appendChild(row);
  }
  table.appendChild(tableBody);
  document.body.appendChild(table);
  table.setAttribute("border", "2");
}
