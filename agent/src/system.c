#include "../include/system.h"
#include "../include/mem.h"
#include <stdio.h>
#include <stdlib.h>
#include <sys/statvfs.h>
#include <sys/sysinfo.h>

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
  return info;
}

/// Returns Null if error ocurrs
struct SystemMetric *getSystemMetric() {
  struct sysinfo mem_info;
  struct statvfs disk_info;
  statvfs("/", &disk_info);
  sysinfo(&mem_info);

  struct SystemMetric *info = malloc(sizeof(struct SystemMetric));
  if (!info) {
    perror("Failed to malloc memory");
    return NULL;
  }
  info->mem_used = getMemAvailable();
  info->disk_used = disk_info.f_bavail * disk_info.f_bsize;
  return info;
}
