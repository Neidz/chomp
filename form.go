package main

import (
	"fmt"
	"strconv"
	"strings"
)

const IntegersFormType = "integersFormType"

type Form struct {
	Selected bool
	Type     string
	Title    string
	RawValue string
}

func NewForm(selected bool, formType, title, rawValue string) Form {
	return Form{Selected: selected, Type: formType, Title: title, RawValue: rawValue}
}

func (f *Form) AddCharacter(char string) {
	f.RawValue += char
}

func ParseFormValueToInts(rawValue string) ([]int, error) {
	allowedChars := "1234567890, "
	valid := containsOnlyAllowedChars(rawValue, allowedChars)

	if !valid {
		return nil, fmt.Errorf("form contains invalid characters, allowed characters: %s", allowedChars)
	}

	cleanRawValue := strings.ReplaceAll(rawValue, ",", " ")
	rawNumbers := strings.Fields(cleanRawValue)

	result := make([]int, len(rawNumbers))
	for i, rawNumber := range rawNumbers {
		number, err := strconv.Atoi(rawNumber)
		if err != nil {
			return nil, fmt.Errorf("form contains invalid number format: %s", rawNumber)
		}
		result[i] = number
	}

	return result, nil
}

func containsOnlyAllowedChars(str, allowed string) bool {
	for _, char := range str {
		if !strings.ContainsRune(allowed, char) {
			return false
		}
	}

	return true
}
