CFLAGS  = -std=c99
CFLAGS += -Wall
CFLAGS += -Wextra

main: 
	cc $(CFLAGS) -o main main.c

main-debug:
	cc -g $(CFLAGS) -o main main.c

.PHONY: run
run: clean main
	./main

.PHONY: lldb
lldb: clean main-debug
	lldb ./main

.PHONY: clean
clean:
	rm -rf *.o *.out *.out.dSYM main test
