package main

import (
	"fmt"
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

var settingsOptions = []string{"todo", "also todo"}

type SettingsModel struct {
	services *Services
	date     *time.Time
	cursor   int
}

func (m SettingsModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "up":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down":
			if m.cursor < len(settingsOptions)-1 {
				m.cursor++
			}
		case "esc":
			return m, SwitchScreen(MainMenuScreen)
		}
	}

	return m, nil
}

func (m SettingsModel) View() string {
	s := "Settings\n\n"

	for i, option := range settingsOptions {
		cursor := " "
		if i == m.cursor {
			cursor = ">"
		}
		s += fmt.Sprintf("%s %s\n", cursor, option)
	}

	return s
}

func InitialSettingsModel(services *Services, date *time.Time) SettingsModel {
	return SettingsModel{
		services: services,
		date:     date,
		cursor:   0,
	}
}

func (m SettingsModel) Init() tea.Cmd {
	return nil
}
