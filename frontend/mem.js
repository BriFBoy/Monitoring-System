import { sys_subscribe, getSysinfo } from "/global/state.js";

// Creates the chart and loads it with information
export default async function mem() {
  const ctx = document.getElementById("mem");
  const sysinfo = getSysinfo();
  let totalMemGB = sysinfo ? sysinfo.mem_total / 1024 / 1024 : 0;

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
          max: totalMemGB || 100,
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

  // subscribes to the sysmetric/sysinfo event
  // Updates the chart when getting new info
  sys_subscribe((sysmetric, sysinfo) => {
    if (!sysmetric || !sysinfo) return;
    let usedGB =
      sysinfo.mem_total / 1024 / 1024 - sysmetric.mem_free / 1024 / 1024;

    const timestamp = new Date().toLocaleTimeString();

    memChart.data.labels.push(timestamp);
    memChart.data.datasets[0].data.push(usedGB);

    if (memChart.data.labels.length > 10) {
      memChart.data.labels.shift();
      memChart.data.datasets[0].data.shift();
    }
    memChart.update();
  });
}
