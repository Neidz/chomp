package main

import (
	"database/sql"
	"fmt"
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

const (
	MainMenuScreen = iota
	CaloriesScreen
	WeightScreen
	SettingsScreen
	ImportDataScreen
)

type SwitchScreenMsg int
type InformationMsg string
type ErrorMsg error
type RefreshStatsMsg struct{}

type Services struct {
	calories CaloriesData
	weight   WeightData
	settings SettingsData
}

type Application struct {
	services     *Services
	activeScreen int
	date         time.Time
	stats        string
	info         *string
	err          error

	mainMenu   MainMenuModel
	calories   CaloriesModel
	weight     WeightModel
	settings   SettingsModel
	importData ImportDataModel
}

func (app Application) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case SwitchScreenMsg:
		app.err = nil
		app.info = nil
		app.activeScreen = int(msg)
		return app, nil
	case InformationMsg:
		msgInfo := string(msg)
		app.info = &msgInfo
		return app, nil
	case ErrorMsg:
		app.err = msg
		return app, nil
	case RefreshStatsMsg:
		stats, err := stats(app.services, app.date)
		if err != nil {
			return app, Error(err)
		}
		app.stats = stats
	case tea.KeyMsg:
		switch msg.String() {
		case "left":
			prevDay := app.date.AddDate(0, 0, -1)
			app.date = prevDay
			app.calories.date = prevDay
			app.weight.date = prevDay
			app.settings.date = prevDay
			app.importData.date = prevDay

			return app, RefreshStats()
		case "right":
			nextDay := app.date.AddDate(0, 0, 1)
			if !nextDay.After(Today()) {
				app.date = nextDay
				app.calories.date = nextDay
				app.weight.date = nextDay
				app.settings.date = nextDay
				app.importData.date = nextDay

				return app, RefreshStats()
			}
		}
	}

	switch app.activeScreen {
	case MainMenuScreen:
		m, cmd := app.mainMenu.Update(msg)
		if mainMenuModel, ok := m.(MainMenuModel); ok {
			app.mainMenu = mainMenuModel
			return app, cmd
		}
	case CaloriesScreen:
		m, cmd := app.calories.Update(msg)
		if caloriesModel, ok := m.(CaloriesModel); ok {
			app.calories = caloriesModel
			return app, cmd
		}
	case WeightScreen:
		m, cmd := app.weight.Update(msg)
		if weightModel, ok := m.(WeightModel); ok {
			app.weight = weightModel
			return app, cmd
		}
	case SettingsScreen:
		m, cmd := app.settings.Update(msg)
		if settingsModel, ok := m.(SettingsModel); ok {
			app.settings = settingsModel
			return app, cmd
		}
	case ImportDataScreen:
		m, cmd := app.importData.Update(msg)
		if importDataModel, ok := m.(ImportDataModel); ok {
			app.importData = importDataModel
			return app, cmd
		}
	}
	return app, nil
}

func (app Application) View() string {
	s := fmt.Sprintf("[%s]\n", app.date.Format(time.DateOnly))
	switch app.activeScreen {
	case MainMenuScreen:
		s += app.mainMenu.View()
	case CaloriesScreen:
		s += app.calories.View()
	case WeightScreen:
		s += app.weight.View()
	case SettingsScreen:
		s += app.settings.View()
	case ImportDataScreen:
		s += app.importData.View()
	}

	if app.info != nil {
		s += fmt.Sprintf("\n\nInfo:\n%s", *app.info)
	}

	if app.err != nil {
		s += fmt.Sprintf("\n\nError:\n%s", app.err.Error())
	}

	s += fmt.Sprintf("\n\n%s", app.stats)

	return s
}

func InitialApp(db *sql.DB) (Application, error) {
	services := Services{
		calories: CaloriesData{
			db: db,
		},
		weight: WeightData{
			db: db,
		},
		settings: SettingsData{
			db: db,
		},
	}

	date := Today()
	stats, err := stats(&services, date)
	if err != nil {
		return Application{}, err
	}

	mainMenuModel := InitialMainMenuModel(&services)
	caloriesModel := InitialCaloriesModel(&services, date)
	weightModel := InitialWeightModel(&services, date)
	settingsModel := InitialSettingsModel(&services, date)
	importDataModel := InitialImportDataModel(&services, date)

	return Application{
		services:     &services,
		activeScreen: 0,
		date:         date,
		stats:        stats,
		info:         nil,
		err:          nil,
		mainMenu:     mainMenuModel,
		calories:     caloriesModel,
		weight:       weightModel,
		settings:     settingsModel,
		importData:   importDataModel,
	}, nil
}

func (app Application) Init() tea.Cmd {
	return tea.ClearScreen
}

func SwitchScreen(newScreen int) func() tea.Msg {
	return func() tea.Msg {
		return SwitchScreenMsg(newScreen)
	}
}

func ShowInformation(information string) func() tea.Msg {
	return func() tea.Msg {
		return InformationMsg(information)
	}
}

func Error(err error) func() tea.Msg {
	return func() tea.Msg {
		return ErrorMsg(err)
	}
}

func RefreshStats() func() tea.Msg {
	return func() tea.Msg {
		return RefreshStatsMsg{}
	}
}

func stats(services *Services, date time.Time) (string, error) {
	targetCals, err := services.settings.ReadTargetCalories()
	if err != nil {
		return "", err
	}
	caloriesStats, err := services.calories.Stats(date, targetCals)
	if err != nil {
		return "", err
	}
	weightStats, err := services.weight.Stats(date)
	if err != nil {
		return "", err
	}
	weeklyChange, err := services.weight.WeeklyChange(date)
	if err != nil {
		return "", err
	}

	stats := ""
	stats += fmt.Sprintf("Calories\n%s\n\n", caloriesStats)
	stats += fmt.Sprintf("Weight\n%s\n%s\n", weightStats, weeklyChange)

	return stats, nil
}
