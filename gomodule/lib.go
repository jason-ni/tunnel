package main

import "C"

import (
	"fmt"
	"os"
	"time"
)

//export Hello
func Hello(fd int64) {
	f := os.NewFile(uintptr(fd), "aaa")
	for {
		time.Sleep(2 * time.Second)
		fmt.Println("----- before go write")
		_, err := f.Write([]byte("hello from go"))
		if err != nil {
			fmt.Printf("==== err: %s\n", err.Error())
		}
	}
}
func main() {}
