const IPLIST = document.getElementById("iplist");

let IPS = fetch(localStorage.getItem("BACKEND") + "/api/getips")
  .then((res) => res.json())
  .then((IPS) => {
    IPS.ips.forEach((item) => {
      let el = document.createElement("a");
      el.innerHTML = item.ip + ":" + item.port;
      el.href = item.ip;
      IPLIST.appendChild(el);
    });
  });
