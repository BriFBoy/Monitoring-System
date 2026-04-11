CFLAGS=-Wall -g

SRCS=$(wildcard src/*.c)
OBJS=$(patsubst src/%.c, obj/%.o, $(SRCS)) 

build: $(OBJS)
	gcc $^ ${CFLAGS} -o bin/monitoring-agent 

obj/%.o: src/%.c 
	gcc ${CFLAGS} -c $^ -o $@ 
clean: 
	rm -rf obj/* bin/*
