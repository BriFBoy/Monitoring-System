#include "../include/parser.h"
#include "../include/request.h"
#include <arpa/inet.h>
#include <netinet/in.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define PORT 8080
#define BUFF_SIZE 1024

void printPacket(const int bytes_rec, const char *buff,
                 const char *addr_print) {
  int len = bytes_rec;
  if (buff[len - 1] == '\n') {
    len--;
  }
  printf("From %s: ", addr_print);
  fwrite(buff, 1, len, stdout);
  printf("\n");
  fflush(stdout);
}

int main(int argc, char *argv[]) {

  struct sockaddr_in addr = {
      .sin_port = htons(PORT), .sin_family = AF_INET, .sin_addr = {INADDR_ANY}};
  struct sockaddr_in pear_addr = {0};
  char *buff = malloc(BUFF_SIZE);
  char *ip_buff = malloc(BUFF_SIZE);

  int socket_fd;
  if ((socket_fd = socket(AF_INET, SOCK_DGRAM, 0)) < 0) {
    perror("Error createing socket\n");
    return EXIT_FAILURE;
  }

  if (bind(socket_fd, (struct sockaddr *)&addr, sizeof(addr))) {
    perror("Error Binding socket\n");
    return EXIT_FAILURE;
  }

  socklen_t pear_len = sizeof(pear_addr);
  while (true) {
    const int bytes_rec = recvfrom(socket_fd, buff, BUFF_SIZE, 0,
                                   (struct sockaddr *)&pear_addr, &pear_len);
    if (bytes_rec <= 0) {
      printf("Did Not get any bytes\n");
      continue;
    }
    buff[bytes_rec] = '\0';

    const char *addr_print =
        inet_ntop(AF_INET, &pear_addr.sin_addr, ip_buff, sizeof(pear_addr));
    if (!addr_print) {
      perror("Failed to convert ip\n");
      return EXIT_FAILURE;
    }
    printPacket(bytes_rec, buff, addr_print);

    struct MetricRequest *metric = parsMetricRequest(buff, bytes_rec);
    printMetricRequest(metric);
    char response[256];
    snprintf(response, sizeof(response), "mem=%d;disk=%d;cpu=%d;", 10000, 900,
             50);
    sendto(socket_fd, response, strlen(response), 0,
           (struct sockaddr *)&pear_addr, sizeof(pear_addr));
  }

  free(buff);
  free(ip_buff);
  return EXIT_SUCCESS;
}
