package main

import (
	"fmt"
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

var importOptions = []string{"import fitnotes data"}

type ImportDataModel struct {
	services         *Services
	date             *time.Time
	cursor           int
	fitnotesDataForm Form
}

func (m ImportDataModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "up":
			if m.cursor > 0 {
				m.cursor--
			}
		case "down":
			if m.cursor < len(importOptions)-1 {
				m.cursor++
			}
		case "esc":
			return m, SwitchScreen(MainMenuScreen)
		case "enter":
			switch importOptions[m.cursor] {
			case "import fitnotes data":
				if m.fitnotesDataForm.Active {
					path := m.fitnotesDataForm.RawValue

					fitnotes, err := LoadFitnotes(path)
					if err != nil {
						return m, Error(err)
					}

					calAdded := 0
					calAlreadyFound := 0
					for _, rec := range fitnotes.calories {
						err := m.services.calories.SafeCreate(rec.date, []int{rec.value})
						switch err {
						case nil:
							calAdded++
						case ErrDateRecordAlreadyExists:
							calAlreadyFound++
						default:
							return m, Error(err)
						}
					}

					weightsAdded := 0
					weightsAlreadyFound := 0
					for _, rec := range fitnotes.weights {
						err := m.services.weight.SafeCreate(rec.date, rec.value)
						switch err {
						case nil:
							weightsAdded++
						case ErrDateRecordAlreadyExists:
							weightsAlreadyFound++
						default:
							return m, Error(err)
						}
					}

					info := fmt.Sprintf("calorie records added: %d\ncalorie records not added (because of existing records): %d\n", calAdded, calAlreadyFound)
					info += fmt.Sprintf("weight records added: %d\nweight records not added (because of existing records): %d\n", weightsAdded, weightsAlreadyFound)

					m.fitnotesDataForm.Reset()
					return m, tea.Batch(RefreshStats(), ShowInformation(info))
				} else {
					m.fitnotesDataForm.Active = true
				}
			}
		case "backspace":
			if m.fitnotesDataForm.Active {
				m.fitnotesDataForm.RemoveCharacter()
			}
		default:
			if m.fitnotesDataForm.Active {
				m.fitnotesDataForm.AddCharacter(msg.String())
			}
		}
	}

	return m, nil
}

func (m ImportDataModel) View() string {
	s := "Import data\n\n"

	formActive := m.fitnotesDataForm.Active

	if m.fitnotesDataForm.Active {
		s += fmt.Sprintf("%s (%s)\n>> %s\n\n\n", m.fitnotesDataForm.Title, m.fitnotesDataForm.Description, m.fitnotesDataForm.RawValue)
	}

	for i, option := range importOptions {
		cursor := " "
		if i == m.cursor && !formActive {
			cursor = ">"
		}
		s += fmt.Sprintf("%s %s\n", cursor, option)
	}

	return s
}

func InitialImportDataModel(services *Services, date *time.Time) ImportDataModel {
	fitnotesDataFormTitle := "Fitnotes path"
	fitnotesDataFormDescription := "provide path to file exported from fitnotes"

	return ImportDataModel{
		services:         services,
		date:             date,
		cursor:           0,
		fitnotesDataForm: NewForm(fitnotesDataFormTitle, fitnotesDataFormDescription),
	}
}

func (m ImportDataModel) Init() tea.Cmd {
	return nil
}
