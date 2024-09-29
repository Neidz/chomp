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

	if firstRow[0] != "Date" || firstRow[2] != "Measurement" || firstRow[3] != "Value" {
		return Fitnotes{}, errors.New("invalid csv format, expected columns: Date Time Measurement Value")
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
		measurement := record[2]
		val := record[3]
		parsedVal, err := strconv.ParseFloat(val, 32)
		if err != nil {
			return Fitnotes{}, err
		}

		if measurement == "Calories" {
			record := FitnotesCaloriesDataRecord{
				date:  date,
				value: int(parsedVal),
			}
			calories = append(calories, record)
		}

		if measurement == "Bodyweight" {
			record := FitnotesWeightDataRecord{
				date:  date,
				value: float32(parsedVal),
			}
			weights = append(weights, record)
		}
	}

	return Fitnotes{
		calories: calories,
		weights:  weights,
	}, nil
}
