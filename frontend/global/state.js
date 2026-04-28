// Subscribers register via sys_subscribe(callback) and receive callbacks
// whenever setSysmetric() is called with new data

let subscribers = [];
let sysmetric_global;
let sysinfo_global;

export function sys_subscribe(callback) {
  subscribers.push(callback);
}

export function getSysmetric() {
  return sysmetric_global;
}

export function getSysinfo() {
  return sysinfo_global;
}

export function notify() {
  subscribers.forEach((callback) => callback(sysmetric_global, sysinfo_global));
}

export function setSysmetric(new_metric) {
  sysmetric_global = new_metric;
  notify();
}

export function setSysinfo(new_info) {
  sysinfo_global = new_info;
}