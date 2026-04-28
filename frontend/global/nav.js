const IPLIST = document.getElementById("iplist");

// Load monitored IPs from backend and render them
fetch(localStorage.getItem("BACKEND") + "/api/getips")
  .then((res) => res.json())
  .then((IPS) => {
    IPS.forEach((item) => {
      IPLIST.appendChild(getIpElement(item));
    });
  });

export function getIpElement(IP) {
  let wrapper = document.createElement("div");
  wrapper.className = "ip-item";

  let el = document.createElement("a");
  el.innerHTML = IP.ip + ":" + IP.port;
  el.href = "/?ip=" + IP.ip + "&port=" + IP.port;

  let del = document.createElement("button");
  del.textContent = "Delete";
  del.className = "delete-button";

  del.onclick = async (e) => {
    e.preventDefault();
    await fetch(localStorage.getItem("BACKEND") + "/api/deleteip", {
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(IP),
    });
    wrapper.remove();
  };

  wrapper.appendChild(el);
  wrapper.appendChild(del);

  return wrapper;
}