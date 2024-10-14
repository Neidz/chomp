package main

import (
	"fmt"

	tea "github.com/charmbracelet/bubbletea"
)

var mainMenuOptions = []string{"calories", "weight", "settings"}

type MainMenuModel struct {
	services *Services
	cursor   int
}

func (m MainMenuModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "up":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down":
			if m.cursor < len(mainMenuOptions)-1 {
				m.cursor++
			}
		case "esc":
			return m, tea.Quit
		case "enter":
			switch mainMenuOptions[m.cursor] {
			case "calories":
				return m, SwitchScreen(CaloriesScreen)
			case "weight":
				return m, SwitchScreen(WeightScreen)
			case "settings":
				return m, SwitchScreen(SettingsScreen)
			}
		}
	}

	return m, nil
}

func (m MainMenuModel) View() string {
	s := "Main Menu\n\n"

	for i, option := range mainMenuOptions {
		cursor := " "
		if i == m.cursor {
			cursor = ">"
		}
		s += fmt.Sprintf("%s %s\n", cursor, option)
	}

	return s
}

func InitialMainMenuModel(services *Services) MainMenuModel {
	return MainMenuModel{
		services: services,
		cursor:   0,
	}
}

func (m MainMenuModel) Init() tea.Cmd {
	return nil
}
