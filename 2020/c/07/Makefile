CFLAGS  = -std=c99
CFLAGS += -Wall
CFLAGS += -Wextra

main: 
	cc -g $(CFLAGS) -o main main.c

main-debug:


.PHONY: run
run: clean main
	./main

.PHONY: clean
clean:
	rm -rf *.o *.out *.out.dSYM main test
