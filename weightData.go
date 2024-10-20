package main

import (
	"database/sql"
	"fmt"
	"time"
)

type WeightData struct {
	db *sql.DB
}

func (w *WeightData) Create(date time.Time, weight float32) error {
	query := `
		INSERT INTO weights (date, weight)
	    VALUES (?1, ?2)`
	args := []any{date.Format(time.DateOnly), weight}

	_, err := w.db.Exec(query, args...)
	if err != nil {
		return err
	}

	return nil
}

func (w *WeightData) SafeCreate(date time.Time, weight float32) error {
	_, err := w.Read(date)
	if err != nil {
		switch err {
		case sql.ErrNoRows:
			err := w.Create(date, weight)
			if err != nil {
				return err
			}
			return nil
		default:
			return err
		}
	}

	return ErrDateRecordAlreadyExists
}

func (w *WeightData) CreateOrUpdate(date time.Time, weight float32) error {
	_, err := w.Read(date)
	if err != nil {
		switch err {
		case sql.ErrNoRows:
			err := w.Create(date, weight)
			if err != nil {
				return err
			}
		default:
			return err
		}
	}

	err = w.Update(date, weight)
	if err != nil {
		return err
	}

	return nil
}

func (w *WeightData) Read(date time.Time) (float32, error) {
	query := `
		SELECT weight
		FROM weights
		WHERE date = ?1`
	args := []any{date.Format(time.DateOnly)}

	var weight float32
	err := w.db.QueryRow(query, args...).Scan(&weight)
	if err != nil {
		return 0, err
	}

	return weight, nil
}

func (w *WeightData) ReadForDates(dates []time.Time) ([]float32, error) {
	query := `
		SELECT weight
		FROM weights
		WHERE date IN ` + sqlArrayPlaceholders(len(dates))
	args := make([]interface{}, len(dates))
	for k, date := range dates {
		args[k] = date.Format(time.DateOnly)
	}

	rows, err := w.db.Query(query, args...)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var weights []float32
	for rows.Next() {
		var weight float32
		err := rows.Scan(&weight)
		if err != nil {
			return nil, err
		}

		weights = append(weights, weight)
	}

	if len(weights) == 0 {
		return nil, ErrDateRangeNoWeightData
	}

	return weights, nil
}

func (w *WeightData) Update(date time.Time, weight float32) error {
	query := `
		UPDATE weights
		SET weight = ?1
		WHERE date = ?2`
	args := []any{weight, date.Format(time.DateOnly)}

	_, err := w.db.Exec(query, args...)
	if err != nil {
		return err
	}

	return nil
}

func (w *WeightData) Delete(date time.Time) error {
	query := `
		DELETE FROM weights
		WHERE date = ?1`
	args := []any{date.Format(time.DateOnly)}

	_, err := w.db.Exec(query, args...)
	if err != nil {
		return err
	}

	return nil
}

func (w *WeightData) Stats(date time.Time) (string, error) {
	weight, err := w.Read(date)
	if err != nil {
		switch err {
		case sql.ErrNoRows:
			return "amount: -\n\n", nil
		default:
			return "", err
		}
	}

	return fmt.Sprintf("amount: %.1f", weight), nil
}

func (w *WeightData) WeeklyChange(calculateBeforeDate time.Time) (string, error) {
	lastWeekStart := calculateBeforeDate.AddDate(0, 0, -8)
	lastWeekEnd := calculateBeforeDate.AddDate(0, 0, -1)
	weekBeforeLastStart := calculateBeforeDate.AddDate(0, 0, -9-7)
	weekBeforeLastEnd := calculateBeforeDate.AddDate(0, 0, -9)

	lastWeekAvg, err := w.averageFromDateRange(lastWeekStart, lastWeekEnd)
	if err != nil {
		switch err {
		case ErrDateRangeNoWeightData:
			return "- (not enough data to calculate)", nil
		default:
			return "", err
		}
	}
	weekBeforeLastAvg, err := w.averageFromDateRange(weekBeforeLastStart, weekBeforeLastEnd)
	if err != nil {
		switch err {
		case ErrDateRangeNoWeightData:
			return "- (not enough data to calculate)", nil
		default:
			return "", err
		}
	}

	return fmt.Sprintf("weekly change: %.1fkg w/w", lastWeekAvg-weekBeforeLastAvg), nil
}

func (w *WeightData) averageFromDateRange(startDate time.Time, endDate time.Time) (float32, error) {
	switch endDate.Compare(startDate) {
	case 0:
		return 0, fmt.Errorf("invalid range, endDate has to be different than startDate")
	case -1:
		return 0, fmt.Errorf("invalid range, endDate has to be date after startDate")
	}

	timeBetween := endDate.Sub(startDate)
	daysBetween := int(timeBetween.Hours()/24) + 1

	dates := make([]time.Time, daysBetween)
	for i := 0; i < len(dates); i++ {
		dates[i] = startDate.AddDate(0, 0, i)
	}

	weightsForRange, err := w.ReadForDates(dates)
	if err != nil {
		return 0, err
	}

	total := float32(0)
	for _, weight := range weightsForRange {
		total += weight
	}

	return total / float32(len(weightsForRange)), nil
}
