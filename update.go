package main

import tea "github.com/charmbracelet/bubbletea"

func (app application) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "up":
			if app.cursor > 0 {
				app.cursor--
			}
		case "down":
			if app.cursor < len(app.options)-1 {
				app.cursor++
			}
		case "q":
			return app, tea.Quit
		case "esc":
			if app.screen == "caloriesMenu" || app.screen == "weightMenu" || app.screen == "settingsMenu" {
				app.screen = "mainMenu"
				app.options = mainMenuOptions
				app.cursor = 0
			}
		default:
			switch app.screen {
			case "mainMenu":
				return app.handleMainMenuUpdate(msg.String())
			case "caloriesMenu":
				return app.handleCaloriesUpdate(msg.String())
			case "weightMenu":
				return app.handleWeightUpdate(msg.String())
			case "settingsMenu":
				return app.handleSettingsUpdate(msg.String())
			}
		}
	}

	return app, nil
}

func (app application) handleMainMenuUpdate(key string) (tea.Model, tea.Cmd) {
	switch key {
	case "enter":
		switch app.options[app.cursor] {
		case "calories":
			app.screen = "caloriesMenu"
			app.options = caloriesMenuOptions
		case "weight":
			app.screen = "weightMenu"
			app.options = weightMenuOptions
		case "settings":
			app.screen = "settingsMenu"
			app.options = settingsMenuOptions
		}
		app.cursor = 0
	}

	return app, nil
}

func (app application) handleCaloriesUpdate(key string) (tea.Model, tea.Cmd) {
	switch key {
	case "enter":
		switch app.options[app.cursor] {
		case "add":
		case "clear":
			err := app.calories.Delete(app.date)
			if err != nil {
				app.error = err
			}
		case "fill":
			err := app.calories.Fill(app.date)
			if err != nil {
				app.error = err
			}
		case "pop":
			err := app.calories.Pop(app.date)
			if err != nil {
				app.error = err
			}
		}
	}

	return app, nil
}

func (app application) handleWeightUpdate(key string) (tea.Model, tea.Cmd) {
	switch key {
	case "enter":
		switch app.options[app.cursor] {
		case "set":
		case "clear":
			err := app.weight.Delete(app.date)
			if err != nil {
				app.error = err
			}
		}
	}

	return app, nil
}

func (app application) handleSettingsUpdate(key string) (tea.Model, tea.Cmd) {
	switch key {
	case "enter":
		switch app.options[app.cursor] {

		}
	}

	return app, nil
}
