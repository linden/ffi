# FFI
Convert to & from `Box`ed Rust types in C for use with [mike](https://github.com/linden/mike).

## Example
```c
#include <stdio.h>

void* create_string(char*);
char* create_cstring(void*);

int main() {
	char* example = "hello world";

	void* pointer = create_string(example);
	char* source = create_cstring(pointer);

	printf("%s -> %p -> %s\n", example, pointer, source);

	return 0;
}
```