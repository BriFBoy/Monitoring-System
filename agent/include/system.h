#ifndef SYSTEM_H
#define SYSTEM_H

// Static system information - populated once at startup
// - mem_total: Total physical memory in bytes
// - disk_total: Total disk space in bytes
// - hostname: Machine hostname string
// - distro: Operating system distro name
struct SystemInfo {
  unsigned long mem_total;
  unsigned long disk_total;
  char *hostname;
  char *distro;
};

// Dynamic system metrics - polled periodically
// - mem_used: Used/available memory in bytes
// - disk_used: Used disk space in bytes
// - cpu: CPU usage percentage (0-100)
// - uptime: System uptime in seconds
struct SystemMetric {
  unsigned long mem_used;
  unsigned long disk_used;
  int cpu;
  float uptime;
};

struct SystemInfo *getSystemInfo();
struct SystemMetric *getSystemMetric();
#endif // !SYSTEM_H