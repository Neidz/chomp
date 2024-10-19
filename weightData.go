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
			return "weight: -\n\n", nil
		default:
			return "", err
		}
	}

	return fmt.Sprintf("weight: %.1f\n\n", weight), nil
}
