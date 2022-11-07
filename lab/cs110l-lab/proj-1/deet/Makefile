SRCS = $(wildcard samples/*.c)
PROGS = $(patsubst %.c,%,$(SRCS))

all: $(PROGS)

%: %.c
	# $(CC) $(CFLAGS) -O0 -g -no-pie -fno-omit-frame-pointer -o $@ $<
	$(CC) $(CFLAGS) -O0 -g -o $@ $<

clean:
	rm -f $(PROGS)
