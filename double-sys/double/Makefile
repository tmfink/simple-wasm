LIBS = double.so double.a

.phony: all clean
all: double.a

double.a: double.o
	ar rcs $@ $<

clean:
	rm -f $(LIBS) *.o
