import init, { generate_kundli } from "./pkg/kundli_core.js";

await init();

const resultsSection = document.getElementById("results");
const divisionSelect = document.getElementById("division");

// Populate divisions
for (let i = 1; i <= 30; i++) {
  const opt = document.createElement("option");
  opt.value = i;
  opt.textContent = `D${i}`;
  divisionSelect.appendChild(opt);
}

document.getElementById("generate").onclick = () => {
  const date = document.getElementById("date").value;
  const time = document.getElementById("time").value;

  if (!date || !time) {
    alert("Please enter date and time");
    return;
  }

  const [year, month, day] = date.split("-").map(Number);
  const [hour, minute] = time.split(":").map(Number);

  const sec = +document.getElementById("sec").value;
  const tz = +document.getElementById("tz").value;
  const lat = +document.getElementById("lat").value;
  const lon = +document.getElementById("lon").value;

  const chart = generate_kundli(
    year,
    month,
    day,
    hour,
    minute,
    sec,
    tz,
    lat,
    lon,
  );

  window.lastChart = chart;

  resultsSection.style.display = "block";

  renderHouses(chart);
  renderPlanets(chart);
  renderDivisional(chart, +divisionSelect.value);
  renderDashas(chart);
};

divisionSelect.onchange = () => {
  if (window.lastChart) {
    renderDivisional(window.lastChart, +divisionSelect.value);
  }
};

// ---------------- HOUSES ----------------

function renderHouses(chart) {
  let html = `
    <p><strong>Ascendant:</strong> ${chart.ascendant_sidereal_deg.toFixed(2)}°</p>
    <table>
      <tr><th>House</th><th>Sign</th></tr>
  `;

  for (const h of chart.houses) {
    html += `<tr><td>${h.number}</td><td>${h.sign}</td></tr>`;
  }

  html += "</table>";
  document.getElementById("houses").innerHTML = html;
}

// ---------------- PLANETS ----------------

function renderPlanets(chart) {
  let html = `
    <table>
      <tr>
        <th>Planet</th>
        <th>Sidereal (°)</th>
        <th>Tropical (°)</th>
      </tr>
  `;

  for (const p of chart.planets) {
    html += `
      <tr>
        <td>${p.name}</td>
        <td>${p.sidereal_deg.toFixed(6)}</td>
        <td>${p.tropical_deg.toFixed(6)}</td>
      </tr>
    `;
  }

  html += "</table>";
  document.getElementById("planets").innerHTML = html;
}

// ---------------- DIVISIONAL ----------------

function renderDivisional(chart, div) {
  const d = chart.divisional_charts.find((c) => c.division === div);
  if (!d) return;

  let html = `
    <table>
      <tr><th>Planet</th><th>Sign</th><th>Degree</th></tr>
  `;

  for (const p of d.planets) {
    html += `
      <tr>
        <td>${p.planet}</td>
        <td>${p.sign}</td>
        <td>${p.degree.toFixed(2)}°</td>
      </tr>
    `;
  }

  html += "</table>";
  document.getElementById("divisional").innerHTML = html;
}

// ---------------- DASHAS (TREE STYLE) ----------------

function renderDashas(chart) {
  const today = new Date().toISOString().slice(0, 10);
  let html = "";

  for (const maha of chart.mahadashas) {
    const isCurrentMaha = today >= maha.start_date && today <= maha.end_date;

    html += `
      <details ${isCurrentMaha ? "open" : ""}>
        <summary><strong>Mahā:</strong> ${maha.maha}
        (${maha.start_date} → ${maha.end_date})</summary>
    `;

    const antars = chart.antardashas.filter((a) => a.maha === maha.maha);

    for (const antar of antars) {
      const isCurrentAntar =
        today >= antar.start_date && today <= antar.end_date;

      html += `
        <details style="margin-left:20px" ${isCurrentAntar ? "open" : ""}>
          <summary>Antar: ${antar.antara}
          (${antar.start_date} → ${antar.end_date})</summary>
          <table style="margin-left:20px">
            <tr><th>Praty</th><th>From</th><th>To</th></tr>
      `;

      const pratys = chart.pratyantardashas.filter(
        (p) => p.maha === maha.maha && p.antara === antar.antara,
      );

      for (const p of pratys) {
        const isCurrentPraty = today >= p.start_date && today <= p.end_date;

        html += `
          <tr class="${isCurrentPraty ? "current" : ""}">
            <td>${p.praty}</td>
            <td>${p.start_date}</td>
            <td>${p.end_date}</td>
          </tr>
        `;
      }

      html += "</table></details>";
    }

    html += "</details>";
  }

  document.getElementById("dashas").innerHTML = html;
}
