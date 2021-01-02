# note: this is all "bad" and "hard-coded"

.PHONY: is clean

is:
	gcc -c src/calgos/insertionsort.c -o object/insertionsort.o -m64 -Ofast -march=native -funroll-loops
	ar rcs object/insertionsort.lib object/insertionsort.o
	#gcc-ar rcs src/calgos/insertionsort.lib --plugin=$(gcc --print-file-name=liblto_plugin.dll.a) src/calgos/insertionsort.o
	#ranlib src/calgos/insertionsort.lib

clean:
	rm object/insertionsort.lib object/insertionsort.o
