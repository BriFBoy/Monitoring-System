let IPLIST1 = document.getElementById("iplist");
const CREATEBUTTON = document.getElementById("create");
const IPELEMENT = document.getElementById("ip");
const PORTELEMENT = document.getElementById("port");
const IPARRAY_NAME = "IPS";

/*
 * let ip = {
 *    ip: string,
 *    port: int,
 * }
 */

CREATEBUTTON.addEventListener("click", async () => {
  const IP = {
    ip: IPELEMENT.value,
    port: Number(PORTELEMENT.value),
  };

  if (!localStorage.getItem(IPARRAY_NAME)) {
    let res = await fetch(localStorage.getItem("BACKEND") + "/api/getips");
    let IPS = await res.json();

    console.log(IPS);
    IPS.push(IP);

    localStorage.setItem(IPARRAY_NAME, JSON.stringify(IPS));

    console.log(IPS);
  } else {
    let IPs = JSON.parse(localStorage.getItem(IPARRAY_NAME));
    IPs.push(IP);
    localStorage.setItem(IPARRAY_NAME, JSON.stringify(IPs));
  }

  fetch(localStorage.getItem("BACKEND") + "/api/addip", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      ip: IP.ip,
      port: IP.port,
    }),
  }).then(() => addIpToNav(IP.ip));
});

function addIpToNav(ip) {
  let el = document.createElement("a");
  el.innerHTML = ip;
  el.href = ip;
  IPLIST1.appendChild(el);
}
