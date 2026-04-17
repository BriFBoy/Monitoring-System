#include "../include/system.h"
#include "../include/cpu.h"
#include "../include/info.h"
#include "../include/mem.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/statvfs.h>
#include <sys/sysinfo.h>
#include <unistd.h>

/// Returns Null if error ocurrs
struct SystemInfo *getSystemInfo() {
  struct statvfs disk_info;
  statvfs("/", &disk_info);

  struct SystemInfo *info = malloc(sizeof(struct SystemInfo));
  if (!info) {
    perror("Failed to malloc memory");
    return NULL;
  }
  info->mem_total = get_mem_total();
  info->disk_total = disk_info.f_blocks * disk_info.f_bsize;

  info->hostname = hostname();
  if (info->hostname == NULL) {
    info->hostname = malloc(25);
    strncpy(info->hostname, "None", 25);
  }

  return info;
}

/// Returns Null if error ocurrs
struct SystemMetric *getSystemMetric() {
  struct sysinfo mem_info;
  struct statvfs disk_info;
  statvfs("/", &disk_info);
  sysinfo(&mem_info);

  struct SystemMetric *metric = malloc(sizeof(struct SystemMetric));
  if (!metric) {
    perror("Failed to malloc memory");
    return NULL;
  }
  metric->mem_used = getMemAvailable();
  metric->disk_used = disk_info.f_bavail * disk_info.f_bsize;
  metric->cpu = getCpuUsage();
  metric->uptime = uptime_in_sec();

  // sets the cpu usage to 0 if error ocurrs when calcing the usage
  if (metric->cpu == -1) {
    metric->cpu = 0;
  }

  return metric;
}
