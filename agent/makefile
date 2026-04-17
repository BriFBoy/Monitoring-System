CFLAGS=-Wall -g
CFLAGS_RELEASE=-O3 -Wall -DNDEBUG 

SRCS=$(wildcard src/*.c)
OBJS=$(patsubst src/%.c, obj/%.o, $(SRCS)) 

build: $(OBJS)
	gcc $^ ${CFLAGS} -o bin/monitoring-agent 

obj/%.o: src/%.c 
	gcc ${CFLAGS} -c $^ -o $@ 

release: clean
	$(MAKE) build CFLAGS="$(CFLAGS_RELEASE)"

install: release 
	sudo cp ./bin/monitoring-agent /usr/bin/monitoring-agent

clean: 
	rm -rf obj/* bin/*
