// Set backend URL based on current host, assuming backend runs on same server on port 8081
let host = window.location.hostname;
localStorage.setItem("BACKEND", `http://${host}:8081`);