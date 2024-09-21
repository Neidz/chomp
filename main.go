package main

import (
	"errors"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"time"
)

type config struct {
	date             time.Time
	caloriesDataPath string
	weightDataPath   string
}

func main() {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		panic(fmt.Sprintf("failed to get home directory for config: %s\n", err.Error()))
	}

	today, err := time.Parse(time.DateOnly, time.Now().Format(time.DateOnly))
	if err != nil {
		panic(fmt.Sprintf("failed to create current time: %s\n", err.Error()))
	}

	cfg := config{
		date:             today,
		caloriesDataPath: filepath.Join(homeDir, ".local/share/chomp/calories.json"),
		weightDataPath:   filepath.Join(homeDir, ".local/share/chomp/weight.json"),
	}

	flag.Func("date", "Date for action (YYYY-MM-DD)", func(val string) error {
		parsed, err := time.Parse(time.DateOnly, val)
		if err != nil {
			return err
		}

		cfg.date = parsed

		return nil
	})

	flag.Parse()
	args := flag.Args()

	err = handleCommand(cfg, args)
	if err != nil {
		fmt.Fprintf(os.Stderr, "%s\n", err.Error())
		os.Exit(1)
	}
}

func handleCommand(cfg config, args []string) error {
	if len(args) == 0 {
		return errors.New("command not provided")
	}

	command := args[0]
	subCommand := "default"
	var rawVals []string

	if len(args) > 1 {
		subCommand = args[1]
	}

	if len(args) > 2 {
		rawVals = args[2:]
	}

	switch command {
	case "calories":
		vals := make([]int, len(rawVals))
		for k, v := range rawVals {
			parsed, err := strconv.Atoi(v)
			if err != nil {
				return fmt.Errorf("invalid value for calories, expected integer but got %s", v)
			}
			vals[k] = parsed
		}

		calories, err := LoadCalories(cfg.caloriesDataPath)
		if err != nil {
			return err
		}

		switch subCommand {
		case "default":
		case "get":
		case "add":
			if len(vals) == 0 {
				return fmt.Errorf("calories add requires at least one value but got 0 instead")
			}
			err := calories.Add(cfg.date, vals)
			if err != nil {
				return err
			}
		case "clear":
			err := calories.Delete(cfg.date)
			if err != nil {
				return err
			}
		case "fill":
			err := calories.Fill(cfg.date)
			if err != nil {
				return err
			}
		case "pop":
			err := calories.Pop(cfg.date)
			if err != nil {
				return err
			}
		case "setTarget":
			if len(vals) == 0 {
				return errors.New("missing value for calories setTarget command")
			}
			if len(vals) > 1 {
				return fmt.Errorf("calories setTarget command requires one value but got %d instead", len(vals))
			}
			if vals[0] < 0 {
				return fmt.Errorf("invalid value for calories setTarget command, expected positive integer but got %d instead", vals[0])
			}
			err := calories.SetTarget(vals[0])
			if err != nil {
				return err
			}
		default:
			return fmt.Errorf("unknown calories command")
		}

		stats := calories.Stats(cfg.date)
		fmt.Printf(stats)
	case "weight":
		weight, err := LoadWeight(cfg.weightDataPath)
		if err != nil {
			return err
		}

		switch subCommand {
		case "default":
		case "get":
		case "set":
			if len(rawVals) == 0 {
				return fmt.Errorf("missing value for weight set, expected float")
			}
			if len(rawVals) > 1 {
				return fmt.Errorf("invalid value for weight set, expected one float but found multiple values")
			}

			parsedVal, err := strconv.ParseFloat(rawVals[0], 32)
			if err != nil {
				return fmt.Errorf("invalid value for weight set, expected float but got %s", rawVals[0])
			}
			if parsedVal < 0 {
				return fmt.Errorf("invalid value for weight set command, expected positive float but got %.1f instead", parsedVal)
			}

			err = weight.Set(cfg.date, float32(parsedVal))
			if err != nil {
				return err
			}
		case "clear":
			err := weight.Delete(cfg.date)
			if err != nil {
				return err
			}
		default:
			return fmt.Errorf("unknown weight command")
		}

		stats := weight.Stats(cfg.date)
		fmt.Printf(stats)
	case "help":
	default:
		return errors.New("unknown command")
	}

	return nil
}
