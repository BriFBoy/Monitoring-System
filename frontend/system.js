let sysmetric_global;
let sysinfo_global;
let ip;
async function fetchInfo() {
  let json = await fetch(localStorage.getItem("BACKEND") + "/api/sysinfo")
    .then((res) => res.json())
    .then((json) => json);
  sysinfo_global = {
    mem_total: json.mem_total,
    disk_total: json.disk_total,
  };
  const sysmetric = await fetchData();
  sysmetric_global = sysmetric;
  console.debug(sysinfo_global);
}
// Updates the global variable to the new information
setInterval(async () => {
  const sysmetric = await fetchData();
  if (sysmetric === null) return;
  sysmetric_global = sysmetric;
}, 2000);
async function fetchData() {
  try {
    const json = await fetch(localStorage.getItem("BACKEND") + "/api/sysmetric")
      .then((res) => res.json())
      .then((json) => json);

    return {
      mem_used: json.mem_used,
      disk_used: json.disk_used,
      cpu_usage: json.cpu_usage,
    };
  } catch (err) {
    console.error("Error fetching data:", err);
    return null;
  }
}

async function mem() {
  const ctx = document.getElementById("mem");
  console.debug(sysinfo_global);
  let totalMemGB = sysinfo_global.mem_total / 1024 / 1024 / 1024;
  console.debug(totalMemGB);

  const memChart = new Chart(ctx, {
    type: "line",
    data: {
      labels: Array(10).fill(""),
      datasets: [
        {
          label: "Memory Used",
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
    let usedGB = sysmetric_global.mem_used / 1024 / 1024 / 1024;
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

async function cpu() {
  const ctx = document.getElementById("cpu");

  const cpuChart = new Chart(ctx, {
    type: "line",
    data: {
      labels: Array(10).fill(""),
      datasets: [
        {
          label: "CPU Used",
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

  // Updates the cpu chart every 2 seconds
  setInterval(async () => {
    const cpuload = sysmetric_global.cpu_usage;
    if (cpuload === null) return;

    const timestamp = new Date().toLocaleTimeString();

    cpuChart.data.labels.push(timestamp);
    cpuChart.data.datasets[0].data.push(cpuload);

    if (cpuChart.data.labels.length > 10) {
      cpuChart.data.labels.shift();
      cpuChart.data.datasets[0].data.shift();
    }
    cpuChart.update();
  }, 2000);
}

async function init() {
  await fetchInfo();
  cpu();
  mem();
}
init();
