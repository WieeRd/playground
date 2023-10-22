CC = g++
CXXFLAGS = -O3
OBJS = main.o

main: $(OBJS)
	$(CC) $(CXXFLAGS) $(OBJS) -o main

main.o: main.cpp
	$(CC) $(CXXFLAGS) -c main.cpp

null:
	time ./main ../input.txt > /dev/null

output:
	time ./main ../input.txt > output.txt && cat output.txt | sha256sum

clean:
	rm -f $(OBJS) main output.txt
