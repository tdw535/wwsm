




const backPageButton = document.getElementById("back-page");

backPageButton.textContent = "Back Page";


function getTableValues() {
  fetch("http://localhost:5057/", { method: 'GET' }).then((response) => {
    return response.json();
  });
}

function generateTable() {
  const table = document.createElement("table");
  const tableBody = document.createElement("tbody");


  tableValues = getTableValues();
  console.log(tableValues);

  for (let ii = 0; ii < 2; ii++) {
    const row = document.createElement("tr");

    for (let jj = 0; jj < 2; jj++) {
      const cell = document.createElement("td");
      const cellText = document.createTextNode(`cell in row ${ii}, column ${jj}`);
      cell.appendChild(cellText);
      row.appendChild(cell);
    }
    tableBody.appendChild(row);
  }
  table.appendChild(tableBody);
  document.body.appendChild(table);
  table.setAttribute("border", "2");
}