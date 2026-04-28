#ifndef REQUEST
#define REQUEST

// Request type determines which data to return
// - TYPE_INVALID: Unset/invalid request type
// - INFO: Request for static system info (memory total, disk total, hostname, distro)
// - METRIC: Request for dynamic metrics (memory used, disk used, cpu, uptime)
enum Type {
  TYPE_INVALID,
  INFO,
  METRIC
};

// Controls streaming behavior (currently not fully implemented)
// - AMOUNT_INVALID: Unset/invalid amount
// - STOPPED: Stop streaming metrics
// - ONCE: Send metrics once only
enum Amount {
  AMOUNT_INVALID,
  STOPPED,
  ONCE
};

struct MetricRequest {
  enum Type type;
  enum Amount amount;
};

void initMetricRequest(struct MetricRequest *metricrequest);
void printMetricRequest(const struct MetricRequest *request);
#endif // !REQUEST