package main

import (
	"fmt"

	tea "github.com/charmbracelet/bubbletea"
)

var weightOptions = []string{"set", "clear"}

type WeightModel struct {
	services *Services
	cursor   int
}

func (m WeightModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "up":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down":
			if m.cursor < len(weightOptions)-1 {
				m.cursor++
			}
		case "esc":
			return m, SwitchScreen(MainMenuScreen)
		}
	}

	return m, nil
}

func (m WeightModel) View() string {
	s := ""
	for i, option := range weightOptions {
		cursor := " "
		if i == m.cursor {
			cursor = ">"
		}
		s += fmt.Sprintf("%s %s\n", cursor, option)
	}

	return s
}

func InitialWeightModel(services *Services) WeightModel {
	return WeightModel{
		services: services,
		cursor:   0,
	}
}

func (m WeightModel) Init() tea.Cmd {
	return nil
}
