package main

import (
	"encoding/hex"
	"testing"

	"github.com/ethereum/go-ethereum/common"

	eip "github.com/kilic/eip2537"
)

func Test(t *testing.T) {
	input := make([]byte, 128)
	res, err := eip.PrecompiledContractsBerlinOnly[common.BytesToAddress([]byte{0x12})].Run(input)
	t.Log(hex.EncodeToString(res))
	t.Log(err)
}
