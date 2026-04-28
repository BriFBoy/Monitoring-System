import { getIpElement } from "/global/nav.js";

let IPLIST1 = document.getElementById("iplist");
const CREATEBUTTON = document.getElementById("create");
const IPELEMENT = document.getElementById("ip");
const PORTELEMENT = document.getElementById("port");
const IPARRAY_NAME = "IPS";

CREATEBUTTON.addEventListener("click", async () => {
  const IP = {
    ip: IPELEMENT.value,
    port: Number(PORTELEMENT.value),
  };

  // Sync local cache with backend to avoid stale data
  if (!localStorage.getItem(IPARRAY_NAME)) {
    let res = await fetch(localStorage.getItem("BACKEND") + "/api/getips");
    let IPS = await res.json();
    IPS.push(IP);
    localStorage.setItem(IPARRAY_NAME, JSON.stringify(IPS));
  } else {
    let IPs = JSON.parse(localStorage.getItem(IPARRAY_NAME));
    IPs.push(IP);
    localStorage.setItem(IPARRAY_NAME, JSON.stringify(IPs));
  }

  // Persist to backend and add to UI on success
  fetch(localStorage.getItem("BACKEND") + "/api/addip", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      ip: IP.ip,
      port: IP.port,
    }),
  }).then(() => IPLIST1.appendChild(getIpElement(IP)));
});