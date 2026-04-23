// state.js is used to send information to all subscribers only when the info is updated

let subscribers = [];
let sysmetric_global;
let sysinfo_global;

// Used to subscribe to the event
export function sys_subscribe(callback) {
  subscribers.push(callback);
}

export function getSysmetric() {
  return sysmetric_global;
}

export function getSysinfo() {
  return sysinfo_global;
}

// Runs every subscriber's callback function
export function notify() {
  subscribers.forEach((callback) => callback(sysmetric_global, sysinfo_global));
}

// setts the sysmetric_global to a new value and notifys everyone
export function setSysmetric(new_metric) {
  sysmetric_global = new_metric;
  notify();
}

export function setSysinfo(new_info) {
  sysinfo_global = new_info;
}
