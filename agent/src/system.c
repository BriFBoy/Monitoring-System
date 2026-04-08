#include "../include/system.h"
#include <stdio.h>
#include <stdlib.h>
#include <sys/statvfs.h>
#include <sys/sysinfo.h>

/// Returns Null if error ocurrs
struct SystemInfo *getSystemInfo() {
  struct sysinfo mem_info;
  struct statvfs disk_info;
  statvfs("/", &disk_info);
  sysinfo(&mem_info);

  struct SystemInfo *info = malloc(sizeof(struct SystemInfo));
  if (!info) {
    perror("Failed to malloc memory");
    return NULL;
  }
  info->mem_total = mem_info.totalram * mem_info.mem_unit;
  info->disk_total = disk_info.f_blocks * disk_info.f_bsize;
  return info;
}
