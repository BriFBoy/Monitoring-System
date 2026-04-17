#include <stdio.h>
#include <stdlib.h>

long long uptime_in_sec() {
  FILE *uptime = fopen("/proc/uptime", "r");
  if (uptime == NULL) {
    return 0;
  }
  float uptime_sec;
  float idle;
  fscanf(uptime, "%f %f", &uptime_sec, &idle);

  return uptime_sec + idle;
}

/// Return the hostname of the pc.
/// When failing to get the hostname NULL will the returned
char *hostname() {
  FILE *file = fopen("/etc/hostname", "r");
  if (file == NULL) {
    return NULL;
  }
  char *hostname = malloc(100);
  fscanf(file, "%100s", hostname);

  return hostname;
}
