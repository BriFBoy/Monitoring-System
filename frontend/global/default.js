// Gets the BACKEND IP from the url
// This means the BACKEND must be on the same server as the frontend
let host = window.location.hostname;
localStorage.setItem("BACKEND", `http://${host}:8081`);
