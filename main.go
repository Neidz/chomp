package main

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

var mainMenuOptions = []string{"calories", "weight", "settings"}
var caloriesMenuOptions = []string{"add", "clear", "fill", "pop"}
var weightMenuOptions = []string{"set", "clear"}
var settingsMenuOptions = []string{"todo", "also todo"}

type application struct {
	calories Calories
	weight   Weight

	stats string
	date  time.Time
	error error

	forms map[string]Form

	screen  string
	options []string
	cursor  int
}

func main() {
	err := tui()
	if err != nil {
		fmt.Print(err)
		os.Exit(1)
	}
}

func tui() error {
	app, err := initialApp()
	if err != nil {
		return err
	}

	p := tea.NewProgram(app)
	if _, err := p.Run(); err != nil {
		return err
	}

	return nil
}

func initialApp() (application, error) {
	homeDir, err := os.UserHomeDir()

	caloriesDataPath := filepath.Join(homeDir, ".local/share/chomp/calories-testing.json")
	weightDataPath := filepath.Join(homeDir, ".local/share/chomp/weight-testing.json")

	calories, err := LoadCalories(caloriesDataPath)
	if err != nil {
		return application{}, err
	}

	weight, err := LoadWeight(weightDataPath)
	if err != nil {
		return application{}, err
	}

	date := Today()

	stats := calories.Stats(date)
	stats += "\n"
	stats += weight.Stats(date)

	return application{
		calories: calories,
		weight:   weight,

		stats: stats,
		date:  date,
		error: nil,

		forms: make(map[string]Form),

		screen:  "mainMenu",
		options: mainMenuOptions,
		cursor:  0,
	}, nil
}

func (app application) Init() tea.Cmd {
	return tea.ClearScreen
}

func Today() time.Time {
	today, err := time.Parse(time.DateOnly, time.Now().Format(time.DateOnly))
	if err != nil {
		panic(fmt.Sprintf("failed to create current time: %s\n", err.Error()))
	}

	return today
}

var (
	ErrDateRecordAlreadyExists = errors.New("record for this date already exists")
)
