package main

import (
	"fmt"
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

var weightOptions = []string{"set", "clear"}

type WeightModel struct {
	services *Services
	date     time.Time
	cursor   int
	setForm  Form
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
			if m.setForm.Active {
				m.setForm.Reset()
			} else {
				return m, SwitchScreen(MainMenuScreen)
			}
		case "enter":
			switch weightOptions[m.cursor] {
			case "set":
				if m.setForm.Active {
					parsedWeight, err := ParseFormValueToFloat(m.setForm.RawValue)
					if err != nil {
						return m, Error(err)
					}
					m.services.weight.CreateOrUpdate(m.date, parsedWeight)
					m.setForm.Reset()
					return m, RefreshStats()
				} else {
					m.setForm.Active = true
				}
			case "clear":
				err := m.services.weight.Delete(m.date)
				if err != nil {
					return m, Error(err)
				}
				return m, RefreshStats()
			}
		case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ".", " ":
			if m.setForm.Active {
				m.setForm.AddCharacter(msg.String())
			}
		case "backspace":
			if m.setForm.Active {
				m.setForm.RemoveCharacter()
			}
		}
	}

	return m, nil
}

func (m WeightModel) View() string {
	s := "Weight\n\n"
	formActive := m.setForm.Active

	if m.setForm.Active {
		s += fmt.Sprintf("%s (%s)\n>> %s\n\n\n", m.setForm.Title, m.setForm.Description, m.setForm.RawValue)
	}

	for i, option := range weightOptions {
		cursor := " "
		if i == m.cursor && !formActive {
			cursor = ">"
		}
		s += fmt.Sprintf("%s %s\n", cursor, option)
	}

	return s
}

func InitialWeightModel(services *Services, date time.Time) WeightModel {
	setFormTitle := "Set weight"
	setFormDescription := "provide weight for current day eg. 123.4"

	return WeightModel{
		services: services,
		date:     date,
		cursor:   0,
		setForm:  NewForm(setFormTitle, setFormDescription),
	}
}

func (m WeightModel) Init() tea.Cmd {
	return nil
}
