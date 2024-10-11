package main

import "fmt"

func (app application) View() string {
	s := fmt.Sprintf("%s\n\n", app.stats)

	for i, option := range app.options {
		formName := fmt.Sprintf("%s%s", app.screen, option)
		form, hasForm := app.forms[formName]

		cursor := " "
		if app.cursor == i && !hasForm {
			cursor = ">"
		}
		s += fmt.Sprintf("%s %s\n", cursor, option)

		if hasForm {
			s += viewActiveForm(form)
		}
	}

	if app.error != nil {
		s += fmt.Sprintf("[Error] %s", app.error.Error())
	}
	s += "\nPress q to quit\n"

	return s
}

func viewActiveForm(form Form) string {
	s := "\t"
	if form.Selected {
		s += "> "
	}
	s += fmt.Sprintf("[%s] %s\n", form.Title, form.RawValue)

	return s
}
