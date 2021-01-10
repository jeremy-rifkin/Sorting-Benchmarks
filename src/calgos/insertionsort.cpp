#include <stdint.h>

extern "C" {
	void c_insertionsort(int32_t*, int);
}

void swap(int32_t* array, int i, int j) {
	int32_t tmp = array[j];
	array[j] = array[i];
	array[i] = tmp;
}

void c_insertionsort(int32_t* array, int size) {
	for(int i = 1; i < size; i++) {
		int j = i;
		while(j > 0 && array[j - 1] > array[j]) {
			swap(array, j, j - 1);
			j--;
		}
	}
}

// It would be elegant to just pace a rust slice to the c code. Turns out it's pretty
// straightforward to do too. It would allow the c functions to match the signature of the other
// sorting algorithms. Unfortunately, we can't just plug in the c functions into the rust code
// because c functions are unsafe and require a wrapper regardless.
// I'm hoping LTO can optimize out the wrapper. TODO: verify this happens.
//struct slice_t {
//	int32_t* ptr;
//	int size;
//};
//void c_insertionsort(struct slice_t slice) {
//	int32_t* array = slice.ptr;
//	int size = slice.size;
//	for(int i = 1; i < size; i++) {
//		int j = i;
//		while(j > 0 && array[j - 1] > array[j]) {
//			swap(array, j, j - 1);
//			j--;
//		}
//	}
//}
