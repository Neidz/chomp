package main

import (
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

var caloriesOptions = []string{"add", "clear", "pop", "fill"}

type CaloriesModel struct {
	services *Services
	date     time.Time
	cursor   int
	addForm  Form
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
			if m.addForm.Active {
				m.addForm.Reset()
			} else {
				return m, SwitchScreen(MainMenuScreen)
			}
		case "enter":
			switch caloriesOptions[m.cursor] {
			case "add":
				if m.addForm.Active {
					parsedCalories, err := ParseFormValueToInts(m.addForm.RawValue)
					if err != nil {
						return m, Error(err)
					}
					err = m.services.calories.CreateOrAdd(m.date, parsedCalories)
					if err != nil {
						return m, Error(err)
					}
					m.addForm.Reset()
					return m, RefreshStats()
				} else {
					m.addForm.Active = true
				}
			case "clear":
				err := m.services.calories.Delete(m.date)
				if err != nil {
					return m, Error(err)
				}
				return m, RefreshStats()
			case "pop":
				err := m.services.calories.SafeDeleteLastElement(m.date)
				if err != nil {
					return m, Error(err)
				}
				return m, RefreshStats()
			case "fill":
				targetCalls, err := m.services.settings.ReadTargetCalories()
				if err != nil {
					return m, Error(err)
				}
				err = m.services.calories.Fill(m.date, targetCalls)
				if err != nil {
					return m, Error(err)
				}
				return m, RefreshStats()
			}
		case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ",", " ":
			if m.addForm.Active {
				m.addForm.AddCharacter(msg.String())
			}
		case "backspace":
			if m.addForm.Active {
				m.addForm.RemoveCharacter()
			}
		}
	}

	return m, nil
}

func (m CaloriesModel) View() string {
	s := ""
	s += StyleTitle.Render("Calories")
	s += "\n\n"

	s += formattedForm(m.addForm)
	s += formattedOptions(caloriesOptions, m.cursor, !m.addForm.Active)

	return s
}

func InitialCaloriesModel(services *Services, date time.Time) CaloriesModel {
	addFormTitle := "Add calories"
	addFormDescription := "provide list of calories, you can provide multiple by separating them with space or ,"

	return CaloriesModel{
		services: services,
		date:     date,
		cursor:   0,
		addForm:  NewForm(addFormTitle, addFormDescription),
	}
}

func (m CaloriesModel) Init() tea.Cmd {
	return nil
}
