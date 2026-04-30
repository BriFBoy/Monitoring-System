#include "../include/request.h"
#include <stdio.h>

// Setts the metricrequest to its default values
void initMetricRequest(struct MetricRequest *metricrequest) {
  metricrequest->amount = AMOUNT_INVALID;
  metricrequest->type = TYPE_INVALID;
}

// Prints the MetricRequest
// Used for logging
void printMetricRequest(const struct MetricRequest *request) {
  printf("MetricRequest:\n");
  switch (request->type) {
  case INFO:
    printf("\tType: INFO\n");
    break;
  case TYPE_INVALID:
    break;
  case METRIC:
    printf("\tType: Metric\n");
    break;
  }

  switch (request->amount) {
  case STOPPED:
    printf("\tAmount: STOPPED\n");
    break;
  case ONCE:
    printf("\tAmount: ONCE\n");
    break;
  case AMOUNT_INVALID:
    break;
  }
}
