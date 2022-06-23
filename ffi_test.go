package ffi

import (
	"testing"
)

func TestString(test *testing.T) {
	converted := CreateCString(CreateString("Hello World"))

	if converted != "Hello World" {
		test.Errorf("expected \"Hello World\", found \"%s\"", converted)
	}

	test.Logf("converted %s", converted)
}

func TestStr(test *testing.T) {
	converted := CreateCStr(CreateStr("Hello World"))

	if converted != "Hello World" {
		test.Errorf("expected \"Hello World\", found \"%s\"", converted)
	}

	test.Logf("converted %s", converted)
}

func TestInt(test *testing.T) {
	converted := CreateFromU8(CreateU8(25))

	if converted != 25 {
		test.Errorf("expected 25, found %d", converted)
	}

	test.Logf("converted %d", converted)
}

func TestArray(test *testing.T) {
	_ = CreateU8Array([]uint{10, 20, 30})
}
