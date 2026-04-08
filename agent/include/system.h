#ifndef SYSTEM_H
#define SYSTEM_H

struct SystemInfo {
  unsigned long mem_total;
  unsigned long disk_total;
};

struct SystemInfo *getSystemInfo();
#endif // !SYSTEM_H
