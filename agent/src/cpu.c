#include <stdio.h>
#include <unistd.h>

typedef struct {
  long long user, nice, system, idle, iowait, irq, softirq, steal, guest,
      guest_nice;
} CpuStats;

// Global variable used to get the cpu usage.
// Writing to this variable can/will cause a race condition!
static float cpu_usage;

void setCpuUsage(float usage) { cpu_usage = usage; }
float getCpuUsage() { return cpu_usage; }

// Calculates CPU usage by reading /proc/stat twice with a 1-second delay,
// then computing the delta in idle time versus total time.
// Returns a float from 0-100 representing percentage CPU usage, or -1 on error.
// Note: This function itself introduces a 1-second blocking delay.
float calcCpuUsage() {
  FILE *fp = fopen("/proc/stat", "r");
  if (!fp)
    return -1.0f;

  CpuStats curr = {0};
  CpuStats last = {0};

  fscanf(fp, "cpu %lld %lld %lld %lld %lld %lld %lld %lld %lld %lld",
         &curr.user, &curr.nice, &curr.system, &curr.idle, &curr.iowait,
         &curr.irq, &curr.softirq, &curr.steal, &curr.guest, &curr.guest_nice);
  fclose(fp);

  sleep(1);

  fp = fopen("/proc/stat", "r");
  fscanf(fp, "cpu %lld %lld %lld %lld %lld %lld %lld %lld %lld %lld",
         &last.user, &last.nice, &last.system, &last.idle, &last.iowait,
         &last.irq, &last.softirq, &last.steal, &last.guest, &last.guest_nice);
  fclose(fp);

  long long total_delta_1 = curr.user + curr.nice + curr.system + curr.idle +
                            curr.iowait + curr.irq + curr.softirq + curr.steal +
                            curr.guest + curr.guest_nice;
  long long idle1 = curr.iowait + curr.idle;

  long long total_delta_2 = last.user + last.nice + last.system + last.idle +
                            last.iowait + last.irq + last.softirq + last.steal +
                            last.guest + last.guest_nice;
  long long idle2 = last.iowait + last.idle;

  return (
      float)((1.0 - (double)(idle2 - idle1) / (total_delta_2 - total_delta_1)) *
             100.0);
}