#ifndef REQUEST
#define REQUEST

enum Type { TYPE_INVALID, INFO, METRIC };
enum Amount { AMOUNT_INVALID, STOPPED, ONCE };

struct MetricRequest {
  enum Type type;
  enum Amount amount;
};

void initMetricRequest(struct MetricRequest *metricrequest);
void printMetricRequest(const struct MetricRequest *request);
#endif // !REQUEST
