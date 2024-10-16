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
)

type SwitchScreenMsg int
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
	date         *time.Time
	stats        string
	err          error

	mainMenu MainMenuModel
	calories CaloriesModel
	weight   WeightModel
	settings SettingsModel
}

func (app Application) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case SwitchScreenMsg:
		app.activeScreen = int(msg)
		return app, nil
	case ErrorMsg:
		app.err = msg
		return app, nil
	case RefreshStatsMsg:
		targetCals, err := app.services.settings.ReadTargetCalories()
		if err != nil {
			return app, Error(err)
		}
		stats, err := app.services.calories.Stats(*app.date, targetCals)
		if err != nil {
			return app, Error(err)
		}
		weightStats, err := app.services.weight.Stats(*app.date)
		if err != nil {
			return app, Error(err)
		}
		stats += weightStats
		app.stats = stats
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
	}
	return app, nil
}

func (app Application) View() string {
	s := ""
	switch app.activeScreen {
	case MainMenuScreen:
		s += app.mainMenu.View()
	case CaloriesScreen:
		s += app.calories.View()
	case WeightScreen:
		s += app.weight.View()
	case SettingsScreen:
		s += app.settings.View()
	}

	if app.err != nil {
		s += fmt.Sprintf("\n\nError: %s", app.err.Error())
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
	targetCals, err := services.settings.ReadTargetCalories()
	if err != nil {
		return Application{}, err
	}

	stats, err := services.calories.Stats(date, targetCals)
	if err != nil {
		return Application{}, err
	}
	weightStats, err := services.weight.Stats(date)
	if err != nil {
		return Application{}, err
	}
	stats += weightStats

	mainMenuModel := InitialMainMenuModel(&services)
	caloriesModel := InitialCaloriesModel(&services, &date)
	weightModel := InitialWeightModel(&services, &date)
	settingsModel := InitialSettingsModel(&services, &date)

	return Application{
		services:     &services,
		activeScreen: 0,
		date:         &date,
		stats:        stats,
		err:          nil,
		mainMenu:     mainMenuModel,
		calories:     caloriesModel,
		weight:       weightModel,
		settings:     settingsModel,
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
