#include "../include/request.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void cmpKeys(const char *key, const char *value,
                    struct MetricRequest *metricrequest) {
  if (strcmp(key, "type") == 0) {
    if (strcmp(value, "info") == 0) {
      metricrequest->type = INFO;
    } else if (strcmp(value, "metric") == 0) {
      metricrequest->type = METRIC;
    }
  } else if (strcmp(key, "amount") == 0) {
    if (strcmp(value, "stopped") == 0) {
      metricrequest->amount = STOPPED;
    } else if (strcmp(value, "once") == 0) {
      metricrequest->amount = ONCE;
    }
  }
}

struct MetricRequest *parsMetricRequest(const char *raw_request,
                                        const size_t size) {
  struct MetricRequest *metricrequest = malloc(sizeof(struct MetricRequest));
  initMetricRequest(metricrequest);

  char *buff = malloc(size + 1);
  char *savepoint_KeyPair;
  char *savepoint_split;

  strncpy(buff, raw_request, size);
  buff[size] = '\0';

  char *line = strtok_r(buff, ";", &savepoint_KeyPair);
  do {

    char *key = strtok_r(line, "=", &savepoint_split);
    char *value = strtok_r(NULL, "=", &savepoint_split);

    cmpKeys(key, value, metricrequest);
  } while ((line = strtok_r(NULL, ";", &savepoint_KeyPair)) != NULL);
  free(buff);
  return metricrequest;
}
