#include <stdio.h>
#include <string.h>

unsigned long get_mem_total() {
  FILE *f = fopen("/proc/meminfo", "r");
  if (!f)
    return 0;

  char key[64];
  unsigned long value;
  char unit[32];

  fscanf(f, "%63s %lu %31s", key, &value, unit);
  if (strcmp(key, "MemTotal:") == 0) {
    fclose(f);
    printf("TotalMem: %lu", value);
    return value;
  }

  fclose(f);
  return 0;
}
unsigned long getMemAvailable() {
  FILE *f = fopen("/proc/meminfo", "r");
  if (!f)
    return 0;

  char key[64];
  unsigned long value;
  char unit[32];

  while (fscanf(f, "%63s %lu %31s", key, &value, unit) == 3) {
    if (strcmp(key, "MemAvailable:") == 0) {
      fclose(f);
      return value;
    }
  }

  fclose(f);
  return 0;
}
