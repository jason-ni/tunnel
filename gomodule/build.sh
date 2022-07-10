#!/bin/bash

#go build -o libgomodule.so -buildmode=c-shared lib.go
go build -o libgomodule.a -buildmode=c-archive lib.go
