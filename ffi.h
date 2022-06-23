#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

void* create_string(char*);
char* create_cstring(void*);

void* create_str(char*);
char* create_cstr(void*);

uint8_t create_from_u8(void*);
void* create_u8(uint8_t);
void print_u8(void*);

uint16_t create_from_u16(void*);
void* create_u16(uint16_t);

uint32_t create_from_u32(void*);
void* create_u32(uint32_t);

uint64_t create_from_u64(void*);
void* create_u64(uint64_t);

size_t create_from_usize(void*);
void* create_usize(size_t);

int8_t create_from_i8(void*);
void* create_i8(int8_t);

int16_t create_from_i16(void*);
void* create_i16(int16_t);

int32_t create_from_i32(void*);
void* create_i32(int32_t);

int64_t create_from_i64(void*);
void* create_i64(int64_t);

ssize_t create_from_isize(void*);
void* create_isize(ssize_t);

void* create_u8_array(uint8_t[], size_t);
void* create_u16_array(uint16_t[], size_t);
void* create_u32_array(uint32_t[], size_t);
void* create_u64_array(uint64_t[], size_t);
void* create_usize_array(size_t[], size_t);

void* create_i8_array(int8_t[], size_t);
void* create_i16_array(int16_t[], size_t);
void* create_i32_array(int32_t[], size_t);
void* create_i64_array(int64_t[], size_t);
void* create_isize_array(ssize_t[], size_t);

void print_u8_array(void*);