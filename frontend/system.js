import mem from "/mem.js";
import cpu from "/cpu.js";
import { setSysmetric, setSysinfo } from "./global/state.js";
import { getSysinfo, getSysmetric } from "./global/state.js";

let ip;
let initialized = false;

// Retries until IP and port appear in URL params, then fetches system info
// and initializes charts. Charts are only started on first successful load.
async function fetchInfo() {
  let params = new URLSearchParams(window.location.search);
  ip = {
    ip: params.get("ip"),
    port: params.get("port"),
  };
  if (ip.port === null || ip.ip === null) {
    console.log("Waiting for ip and port in URL...");
    setTimeout(fetchInfo, 500);
    return;
  }
  console.log("Got ip:", ip.ip, "port:", ip.port);
  let json = await fetch(
    localStorage.getItem("BACKEND") +
      "/api/sysinfo" +
      "?ip=" +
      ip.ip +
      "&port=" +
      ip.port,
  )
    .then((res) => res.json())
    .then((json) => json);
  setSysinfo({
    mem_total: json.mem_total,
    disk_total: json.disk_total,
    hostname: json.hostname,
    distro: json.distro,
  });

  if (!initialized) {
    initialized = true;
    cpu();
    mem();
  }
}

setInterval(async () => {
  if (!ip || !ip.ip || !ip.port) return;
  const sysmetric = await fetchData();
  if (sysmetric === null) return;
  setSysmetric(sysmetric);
  updateStats();
}, 2000);

// Maps backend field names to frontend convention, mem_used from backend becomes mem_free
async function fetchData() {
  if (!ip || !ip.ip || !ip.port) return null;
  try {
    const json = await fetch(
      localStorage.getItem("BACKEND") +
        "/api/sysmetric" +
        "?ip=" +
        ip.ip +
        "&port=" +
        ip.port,
    )
      .then((res) => res.json())
      .then((json) => json);

    return {
      mem_free: json.mem_used,
      disk_used: json.disk_used,
      cpu_usage: json.cpu_usage,
    };
  } catch (err) {
    console.error("Error fetching data:", err);
    return null;
  }
}

function updateStats() {
  const CPUSTAT = document.getElementById("cpu-stat");
  const MEMSTAT = document.getElementById("mem-stat");
  const DISKSTAT = document.getElementById("disk-stat");
  const HOSTNAME = document.getElementById("hostname");
  const DISTRO = document.getElementById("distro");

  let sysmetric = getSysmetric();
  let sysinfo = getSysinfo();

  if (CPUSTAT && sysmetric) CPUSTAT.textContent = sysmetric.cpu_usage;
  if (MEMSTAT && sysinfo && sysmetric) {
    const memUsedGB =
      sysinfo.mem_total / 1024 / 1024 - sysmetric.mem_free / 1024 / 1024;
    MEMSTAT.textContent = memUsedGB.toFixed(1);
  }
  if (DISKSTAT && sysinfo && sysmetric) {
    const diskFreeGB =
      sysinfo.disk_total / 1024 / 1024 - sysmetric.disk_used / 1024 / 1024;
    DISKSTAT.textContent = diskFreeGB.toFixed(1);
  }
  if (HOSTNAME && sysinfo) HOSTNAME.textContent = sysinfo.hostname;
  if (DISTRO && sysinfo) DISTRO.textContent = sysinfo.distro;
}
fetchInfo();

