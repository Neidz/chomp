package main

import (
	"fmt"

	tea "github.com/charmbracelet/bubbletea"
)

var caloriesOptions = []string{"add", "clear", "fill", "pop"}

type CaloriesModel struct {
	services *Services
	cursor   int
}

func (m CaloriesModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "up":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down":
			if m.cursor < len(caloriesOptions)-1 {
				m.cursor++
			}
		case "esc":
			return m, SwitchScreen(MainMenuScreen)
		}
	}

	return m, nil
}

func (m CaloriesModel) View() string {
	s := ""
	for i, option := range caloriesOptions {
		cursor := " "
		if i == m.cursor {
			cursor = ">"
		}
		s += fmt.Sprintf("%s %s\n", cursor, option)
	}

	return s
}

func InitialCaloriesModel(services *Services) CaloriesModel {
	return CaloriesModel{
		services: services,
		cursor:   0,
	}
}

func (m CaloriesModel) Init() tea.Cmd {
	return nil
}
