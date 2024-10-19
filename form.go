package main

import (
	"fmt"
	"strconv"
	"strings"
)

const IntegersFormType = "integersFormType"

type Form struct {
	Active      bool
	Title       string
	Description string
	RawValue    string
}

func NewForm(title, description string) Form {
	return Form{Active: false, Title: title, Description: description, RawValue: ""}
}

func (f *Form) Reset() {
	f.Active = false
	f.RawValue = ""
}

func (f *Form) AddCharacter(char string) {
	f.RawValue += char
}

func (f *Form) RemoveCharacter() {
	if len(f.RawValue) != 0 {
		f.RawValue = f.RawValue[0 : len(f.RawValue)-1]
	}
}

func ParseFormValueToInts(rawValue string) ([]int, error) {
	allowedChars := "1234567890, "
	valid := containsOnlyAllowedChars(rawValue, allowedChars)

	if !valid {
		return nil, fmt.Errorf("form contains invalid characters, allowed characters: %s", allowedChars)
	}

	cleanRawValue := strings.ReplaceAll(rawValue, ",", " ")
	rawNumbers := strings.Fields(cleanRawValue)

	parsed := make([]int, len(rawNumbers))
	for i, rawNumber := range rawNumbers {
		number, err := strconv.Atoi(rawNumber)
		if err != nil {
			return nil, fmt.Errorf("form contains invalid number format, expected '123' or '123, 123' or '123 123' but got: %s", rawNumber)
		}
		parsed[i] = number
	}

	return parsed, nil
}

func ParseFormValueToFloat(rawValue string) (float32, error) {
	allowedChars := "1234567890.,"
	valid := containsOnlyAllowedChars(rawValue, allowedChars)

	if !valid {
		return 0, fmt.Errorf("form contains invalid characters, allowed characters: %s", rawValue)
	}

	parsed, err := strconv.ParseFloat(rawValue, 32)
	if err != nil {
		return 0, fmt.Errorf("form contains invalid number format, expected '123.4' but got: %s", rawValue)
	}

	return float32(parsed), nil
}

func ParseFormValueToInt(rawValue string) (int, error) {
	allowedChars := "1234567890"
	valid := containsOnlyAllowedChars(rawValue, allowedChars)

	if !valid {
		return 0, fmt.Errorf("form contains invalid characters, allowed characters: %s", rawValue)
	}

	parsed, err := strconv.Atoi(rawValue)
	if err != nil {
		return 0, fmt.Errorf("form contains invalid number format, expected '1234' but got: %s", rawValue)
	}

	return parsed, nil
}

func containsOnlyAllowedChars(str, allowed string) bool {
	for _, char := range str {
		if !strings.ContainsRune(allowed, char) {
			return false
		}
	}

	return true
}
