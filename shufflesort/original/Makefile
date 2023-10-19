CC = g++
CXXFLAGS = -O3
OBJS = main.o

main: $(OBJS)
	$(CC) $(CXXFLAGS) $(OBJS) -o main

main.o: main.cpp
	$(CC) $(CXXFLAGS) -c main.cpp

bench:
	time ./main ../input.txt > output.txt

clean:
	rm -f $(OBJS) main output.txt
