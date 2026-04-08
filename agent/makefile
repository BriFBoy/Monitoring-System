CC=gcc
CFLAGS=-Wall -g
OBJ=obj
SRC=src

build: ${OBJ}/main.o ${OBJ}/parser.o ${OBJ}/request.o ${OBJ}/system.o
	${CC} $^ ${CFLAGS} -o bin/monitoring-agent

obj/main.o: ${SRC}/main.c 
	${CC} ${CFLAGS} -c $< -o $@

obj/parser.o: ${SRC}/parser.c
	${CC} ${CFLAGS} -c $< -o $@

obj/request.o: ${SRC}/request.c
	${CC} ${CFLAGS} -c $< -o $@

obj/system.o: ${SRC}/system.c
	${CC} ${CFLAGS} -c $< -o $@
clean: 
	rm -rf obj/* bin/*
