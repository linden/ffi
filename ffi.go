package ffi

//#cgo LDFLAGS: -L. -lffi
//#include "./ffi.h"
import "C"
import "unsafe"

func CreateString(source string) unsafe.Pointer {
	return C.create_string(C.CString(source))
}

func CreateCString(source unsafe.Pointer) string {
	return C.GoString(C.create_cstring(source))
}

func CreateStr(source string) unsafe.Pointer {
	return C.create_str(C.CString(source))
}

func CreateCStr(source unsafe.Pointer) string {
	return C.GoString(C.create_cstr(source))
}

func CreateU8(source uint) unsafe.Pointer {
	return C.create_u8(C.uint8_t(source))
}

func CreateFromU8(source unsafe.Pointer) uint {
	return uint(C.create_from_u8(source))
}

func CreateU16(source uint) unsafe.Pointer {
	return C.create_u16(C.uint16_t(source))
}

func CreateFromU16(source unsafe.Pointer) uint {
	return uint(C.create_from_u16(source))
}

func CreateU32(source uint) unsafe.Pointer {
	return C.create_u32(C.uint32_t(source))
}

func CreateFromU32(source unsafe.Pointer) uint {
	return uint(C.create_from_u32(source))
}

func CreateU64(source uint) unsafe.Pointer {
	return C.create_u64(C.uint64_t(source))
}

func CreateFromU64(source unsafe.Pointer) uint {
	return uint(C.create_from_u64(source))
}

func CreateUSize(source uint) unsafe.Pointer {
	return C.create_usize(C.size_t(source))
}

func CreateFromUSize(source unsafe.Pointer) uint {
	return uint(C.create_from_usize(source))
}

func CreateI8(source int) unsafe.Pointer {
	return C.create_i8(C.int8_t(source))
}

func CreateFromI8(source unsafe.Pointer) int {
	return int(C.create_from_i8(source))
}

func CreateI16(source int) unsafe.Pointer {
	return C.create_i16(C.int16_t(source))
}

func CreateFromI16(source unsafe.Pointer) int {
	return int(C.create_from_i16(source))
}

func CreateI32(source int) unsafe.Pointer {
	return C.create_i32(C.int32_t(source))
}

func CreateFromI32(source unsafe.Pointer) int {
	return int(C.create_from_i32(source))
}

func CreateI64(source int) unsafe.Pointer {
	return C.create_u64(C.uint64_t(source))
}

func CreateFromI64(source unsafe.Pointer) int {
	return int(C.create_from_i64(source))
}

func CreateISize(source int) unsafe.Pointer {
	return C.create_isize(C.ssize_t(source))
}

func CreateFromISize(source unsafe.Pointer) int {
	return int(C.create_from_isize(source))
}

func CreateU8Array(source []uint) unsafe.Pointer {
	return C.create_u8_array((*C.uint8_t)(unsafe.Pointer(&source[0])), C.size_t(len(source)))
}

func CreateU16Array(source []uint) unsafe.Pointer {
	return C.create_u16_array((*C.uint16_t)(unsafe.Pointer(&source[0])), C.size_t(len(source)))
}

func CreateU32Array(source []uint) unsafe.Pointer {
	return C.create_u32_array((*C.uint32_t)(unsafe.Pointer(&source[0])), C.size_t(len(source)))
}

func CreateU64Array(source []uint) unsafe.Pointer {
	return C.create_u64_array((*C.uint64_t)(unsafe.Pointer(&source[0])), C.size_t(len(source)))
}

func CreateUSizeArray(source []uint) unsafe.Pointer {
	return C.create_usize_array((*C.size_t)(unsafe.Pointer(&source[0])), C.size_t(len(source)))
}

func CreateI8Array(source []int) unsafe.Pointer {
	return C.create_i8_array((*C.int8_t)(unsafe.Pointer(&source[0])), C.size_t(len(source)))
}

func CreateI16Array(source []int) unsafe.Pointer {
	return C.create_i16_array((*C.int16_t)(unsafe.Pointer(&source[0])), C.size_t(len(source)))
}

func CreateI32Array(source []int) unsafe.Pointer {
	return C.create_i32_array((*C.int32_t)(unsafe.Pointer(&source[0])), C.size_t(len(source)))
}

func CreateI64Array(source []int) unsafe.Pointer {
	return C.create_i64_array((*C.int64_t)(unsafe.Pointer(&source[0])), C.size_t(len(source)))
}

func CreateISizeArray(source []int) unsafe.Pointer {
	return C.create_isize_array((*C.ssize_t)(unsafe.Pointer(&source[0])), C.size_t(len(source)))
}
