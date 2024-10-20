package main

import (
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

var settingsOptions = []string{"set target calories"}

type SettingsModel struct {
	services              *Services
	date                  time.Time
	cursor                int
	setTargetCaloriesForm Form
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
			if m.setTargetCaloriesForm.Active {
				m.setTargetCaloriesForm.Reset()
			} else {
				return m, SwitchScreen(MainMenuScreen)
			}
		case "enter":
			switch settingsOptions[m.cursor] {
			case "set target calories":
				if m.setTargetCaloriesForm.Active {
					targetCals, err := ParseFormValueToInt(m.setTargetCaloriesForm.RawValue)
					if err != nil {
						return m, Error(err)
					}
					err = m.services.settings.UpdateTargetCalories(targetCals)
					if err != nil {
						return m, Error(err)
					}
					m.setTargetCaloriesForm.Reset()
					return m, RefreshStats()
				} else {
					m.setTargetCaloriesForm.Active = true
				}
			}
		case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9":
			if m.setTargetCaloriesForm.Active {
				m.setTargetCaloriesForm.AddCharacter(msg.String())
			}
		case "backspace":
			if m.setTargetCaloriesForm.Active {
				m.setTargetCaloriesForm.RemoveCharacter()
			}
		}
	}

	return m, nil
}

func (m SettingsModel) View() string {
	s := ""
	s += StyleTitle.Render("Settings")
	s += "\n\n"

	s += formattedForm(m.setTargetCaloriesForm)
	s += formattedOptions(settingsOptions, m.cursor, !m.setTargetCaloriesForm.Active)

	return s
}

func InitialSettingsModel(services *Services, date time.Time) SettingsModel {
	setTargetCaloriesFormTitle := "Target calories"
	setTargetCaloriesFormDescription := "provide your new target for daily calorie intake"

	return SettingsModel{
		services:              services,
		date:                  date,
		cursor:                0,
		setTargetCaloriesForm: NewForm(setTargetCaloriesFormTitle, setTargetCaloriesFormDescription),
	}
}

func (m SettingsModel) Init() tea.Cmd {
	return nil
}
