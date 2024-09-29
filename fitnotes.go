package main

import (
	"encoding/csv"
	"errors"
	"io"
	"os"
	"strconv"
	"time"
)

type FitnotesCaloriesDataRecord struct {
	date  time.Time
	value int
}

type FitnotesWeightDataRecord struct {
	date  time.Time
	value float32
}

type Fitnotes struct {
	calories []FitnotesCaloriesDataRecord
	weights  []FitnotesWeightDataRecord
}

func LoadFitnotes(path string) (Fitnotes, error) {
	f, err := os.Open(path)
	if err != nil {
		return Fitnotes{}, err
	}
	defer f.Close()

	reader := csv.NewReader(f)

	firstRow, err := reader.Read()
	if err == io.EOF {
		return Fitnotes{}, errors.New("empty file")
	}
	if err != nil {
		return Fitnotes{}, err
	}

	if firstRow[0] != "Date" || firstRow[1] != "Measurement" || firstRow[2] != "Value" {
		return Fitnotes{}, errors.New("invalid csv format, expected columns: Date Measurement Value")
	}

	var calories []FitnotesCaloriesDataRecord
	var weights []FitnotesWeightDataRecord

	for {
		record, err := reader.Read()
		if err == io.EOF {
			break
		}
		if err != nil {
			return Fitnotes{}, err
		}

		date, err := time.Parse("2006-01-02", record[0])
		if err != nil {
			return Fitnotes{}, err
		}
		measurement := record[1]
		value := record[2]

		if measurement == "Calories" {
			parsed, err := strconv.ParseInt(value, 10, 32)
			if err != nil {
				return Fitnotes{}, err
			}

			record := FitnotesCaloriesDataRecord{
				date:  date,
				value: int(parsed),
			}
			calories = append(calories, record)
		}

		if measurement == "Bodyweight" {
			parsed, err := strconv.ParseFloat(value, 32)
			if err != nil {
				return Fitnotes{}, err
			}

			record := FitnotesWeightDataRecord{
				date:  date,
				value: float32(parsed),
			}
			weights = append(weights, record)
		}
	}

	return Fitnotes{
		calories: calories,
		weights:  weights,
	}, nil
}
