#!/bin/sh
cd ./eip2537go
rm go.mod
rm go.sum
go get -u
make static_external
