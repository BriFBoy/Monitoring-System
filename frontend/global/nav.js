const IPLIST = document.getElementById("iplist");

let IPS = fetch(localStorage.getItem("BACKEND") + "/api/getips")
  .then((res) => res.json())
  .then((IPS) => {
    IPS.forEach((item) => {
      IPLIST.appendChild(getIpElement(item));
    });
  });

function getIpElement(IP) {
  // wrapper for link + delete button
  let wrapper = document.createElement("div");
  wrapper.className = "ip-item";

  // the link
  let el = document.createElement("a");
  el.innerHTML = IP.ip + ":" + IP.port;
  el.href = "/?ip=" + IP.ip + "&port=" + IP.port;

  // delete button
  let del = document.createElement("button");
  del.textContent = "Delete";
  del.className = "delete-button";

  del.onclick = async (e) => {
    e.preventDefault(); // prevents the <a> from triggering

    // call backend delete API (adjust URL as needed)
    await fetch(localStorage.getItem("BACKEND") + "/api/deleteip", {
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(IP),
    });

    // remove from DOM
    wrapper.remove();
  };

  wrapper.appendChild(el);
  wrapper.appendChild(del);

  return wrapper;
}
