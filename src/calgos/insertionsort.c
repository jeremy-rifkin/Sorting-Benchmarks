#include <stdint.h>
#include <stdio.h>

// compiler will probably ignore the inline specifier, though it should inline this function regardless.
inline void swap(int32_t* array, int i, int j) {
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

/*struct slice_t {
	int32_t* ptr;
	int size;
};
struct slice_t c_slice_test(struct slice_t slice) {
	return slice;
}*/

//struct slice {
//	int32_t* array;
//	int size;
//};
//
//void c_insertionsort_s(struct slice data) {
//	printf("AYYYYY %p %d\n", data.array, data.size);
//}

//void c_insertionsort(struct {int size; int32_t* array;} data) {
//	printf("BEEEEE\n");
//}

//void print_arr(int32_t* array, int size) {
//	for(int i = 0; i < size; i++) {
//		printf("%d ", array[i]);
//	}
//	printf("\n");
//}

void c_test() {
	printf("TEST TEST TEST\n");
	//int arr[] = {5,4,3,2,1};
	//print_arr(arr, 5);
	//c_insertionsort(arr, 5);
	//print_arr(arr, 5);
}
