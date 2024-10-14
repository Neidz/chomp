package main

import (
	"os"
	"path/filepath"

	tea "github.com/charmbracelet/bubbletea"
)

const (
	MainMenuScreen = iota
	CaloriesScreen
	WeightScreen
	SettingsScreen
)

type SwitchScreenMsg int

type Services struct {
	calories Calories
	weight   Weight
}

type Application struct {
	services     Services
	activeScreen int

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
	return s
}

func InitialApp() (Application, error) {
	homeDir, err := os.UserHomeDir()

	caloriesDataPath := filepath.Join(homeDir, ".local/share/chomp/calories-testing.json")
	weightDataPath := filepath.Join(homeDir, ".local/share/chomp/weight-testing.json")

	caloriesData, err := LoadCalories(caloriesDataPath)
	if err != nil {
		return Application{}, err
	}

	weightData, err := LoadWeight(weightDataPath)
	if err != nil {
		return Application{}, err
	}

	services := Services{
		calories: caloriesData,
		weight:   weightData,
	}

	mainMenuModel := InitialMainMenuModel(&services)
	caloriesModel := InitialCaloriesModel(&services)
	weightModel := InitialWeightModel(&services)
	settingsModel := InitialSettingsModel(&services)

	return Application{
		services:     services,
		activeScreen: 0,
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
