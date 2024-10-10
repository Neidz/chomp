package main

import "fmt"

func (app application) View() string {
	s := fmt.Sprintf("%s\n\n", app.stats)

	for i, option := range app.options {
		cursor := " "
		if app.cursor == i {
			cursor = ">"
		}

		s += fmt.Sprintf("%s %s\n", cursor, option)
	}

	if app.error != nil {
		s += fmt.Sprintf("[Error] %s", app.error.Error())
	}

	s += "\nPress q to quit\n"

	return s
}
