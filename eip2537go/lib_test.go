package main

import (
	"encoding/hex"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/vm"
)

func Test(t *testing.T) {
	input := make([]byte, 128)
	res, err := vm.PrecompiledContractsBerlin[common.BytesToAddress([]byte{0x12})].Run(input)
	t.Log(hex.EncodeToString(res))
	t.Log(err)
}
