#include <algorithm>
#include <stdint.h>

extern "C" {
	void std_sort(int32_t*, int);
	//void std_stable_sort(int32_t*, int);
}

void std_sort(int32_t* array, int size) {
	std::sort(array, array + size);
}

// TODO: linker challenges with this
// need to explicitly link with libstdc++ but trouble with that on windows
// and on windows linker also says "fatal error LNK1143: invalid or corrupt file: no symbol for
// COMDAT section 0x7"
//void std_stable_sort(int32_t* array, int size) {
//	std::stable_sort(array, array + size);
//}
