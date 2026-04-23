import { sys_subscribe } from "/global/state.js";

export default async function cpu() {
  const ctx = document.getElementById("cpu");

  const cpuChart = new Chart(ctx, {
    type: "line",
    data: {
      labels: Array(10).fill(""),
      datasets: [
        {
          label: "in %",
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
          max: 100,
          ticks: {
            stepSize: 1,
            callback: (value) => value.toFixed(0) + " %",
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

  sys_subscribe((sysmetric) => {
    if (!sysmetric) return;
    const cpuload = sysmetric.cpu_usage;
    if (cpuload === null) return;

    const timestamp = new Date().toLocaleTimeString();

    cpuChart.data.labels.push(timestamp);
    cpuChart.data.datasets[0].data.push(cpuload);

    if (cpuChart.data.labels.length > 10) {
      cpuChart.data.labels.shift();
      cpuChart.data.datasets[0].data.shift();
    }
    cpuChart.update();
  });
}

