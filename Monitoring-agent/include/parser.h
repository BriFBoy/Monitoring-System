#ifndef PARSER
#define PARSER

#include <stddef.h>
struct MetricRequest *parsMetricRequest(const char *raw_request,
                                        const size_t size);

#endif // !PARSER
