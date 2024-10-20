package main

import "strings"

func sqlArrayPlaceholders(n int) string {
	placeholders := make([]string, n)
	for i := 0; i < n; i++ {
		placeholders[i] = "?"
	}

	return "(" + strings.Join(placeholders, ",") + ")"
}
