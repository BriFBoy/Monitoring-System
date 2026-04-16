import { sysinfo_global, sysmetric_global } from "/system.js";

export default async function mem() {
  const ctx = document.getElementById("mem");
  console.debug(sysinfo_global);
  let totalMemGB = sysinfo_global.mem_total / 1024 / 1024;
  console.debug(totalMemGB);

  const memChart = new Chart(ctx, {
    type: "line",
    data: {
      labels: Array(10).fill(""),
      datasets: [
        {
          label: "Memory",
          data: Array(10).fill(0),
          backgroundColor: "rgb(0, 0, 255)",
          borderColor: "rgb(0, 0, 255)",
          tension: 0.2,
          pointRadius: 3,
        },
      ],
    },
    options: {
      responsive: true,
      animation: false,
      maintainAspectRatio: false,
      scales: {
        y: {
          beginAtZero: true,
          max: totalMemGB,
          title: {
            display: true,
            text: "Memory (GB)",
          },
          ticks: {
            stepSize: 1,
            callback: (value) => value.toFixed(0) + " GB",
            font: { size: 16 },
          },
        },
        x: {
          title: {
            display: true,
          },
        },
      },
    },
  });

  // Fetch memory info from your server

  // Update chart every second
  setInterval(() => {
    if (!sysmetric_global) return;
    let usedGB =
      sysinfo_global.mem_total / 1024 / 1024 -
      sysmetric_global.mem_free / 1024 / 1024;
    if (usedGB === null) return;

    const timestamp = new Date().toLocaleTimeString();

    memChart.data.labels.push(timestamp);
    memChart.data.datasets[0].data.push(usedGB);

    if (memChart.data.labels.length > 10) {
      memChart.data.labels.shift();
      memChart.data.datasets[0].data.shift();
    }
    memChart.update();
  }, 2000);
}
