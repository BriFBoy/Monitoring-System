#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Read the uptime from the virtual file and return it
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

// Reads the PRETTY_NAME value from /etc/os-release (e.g., "Ubuntu 22.04 LTS")
// by finding the PRETTY_NAME= line and stripping quotes around the value
char *distro() {
  FILE *f = fopen("/etc/os-release", "r");
  if (!f) {
    return NULL;
  }

  char line[256];
  char *name = malloc(100);
  name[99] = '\0';

  while (fgets(line, sizeof(line), f)) {
    if (strncmp(line, "PRETTY_NAME=", 12) == 0) {
      // Remove PRETTY_NAME= and surrounding quotes
      char *value = line + 12;
      if (*value == '"')
        value++;
      char *end = strchr(value, '"');
      if (end)
        *end = '\0';
      strncpy(name, value, 100 - 1);
      name[99] = '\0';
      break;
    }
  }

  fclose(f);

  return name;
}