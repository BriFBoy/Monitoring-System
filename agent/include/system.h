#ifndef SYSTEM_H
#define SYSTEM_H

struct SystemInfo {
  unsigned long mem_total;
  unsigned long disk_total;
};
struct SystemMetric {
  unsigned long mem_used;
  unsigned long disk_used;
  int cpu;
};

struct SystemInfo *getSystemInfo();
struct SystemMetric *getSystemMetric();
#endif // !SYSTEM_H
