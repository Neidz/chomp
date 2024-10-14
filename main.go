package main

import (
	"errors"
	"fmt"
	"os"
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

func main() {
	err := tui()
	if err != nil {
		fmt.Print(err)
		os.Exit(1)
	}
}

func tui() error {
	app, err := InitialApp()
	if err != nil {
		return err
	}

	p := tea.NewProgram(app)
	if _, err := p.Run(); err != nil {
		return err
	}

	return nil
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
