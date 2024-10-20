package main

import (
	"database/sql"
	"errors"
	"fmt"
	"strconv"
	"strings"
	"time"
)

type CaloriesData struct {
	db *sql.DB
}

func (c *CaloriesData) Create(date time.Time, cals []int) error {
	query := `
		INSERT INTO calories (date, calories)
	    VALUES (?1, ?2)`
	args := []any{date.Format(time.DateOnly), serializeCalories(cals)}

	_, err := c.db.Exec(query, args...)
	if err != nil {
		return err
	}

	return nil
}

func (c *CaloriesData) SafeCreate(date time.Time, cals []int) error {
	_, err := c.Read(date)
	if err != nil {
		switch err {
		case sql.ErrNoRows:
			err := c.Create(date, cals)
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

func (c *CaloriesData) CreateOrAdd(date time.Time, cals []int) error {
	_, err := c.Read(date)
	if err != nil {
		switch err {
		case sql.ErrNoRows:
			err := c.Create(date, cals)
			if err != nil {
				return err
			}
			return nil
		default:
			return err
		}
	}

	c.add(date, cals)

	return nil
}

func (c *CaloriesData) Read(date time.Time) ([]int, error) {
	query := `
		SELECT calories
		FROM calories
		WHERE date = ?1`
	args := []any{date.Format(time.DateOnly)}

	var sCals string
	err := c.db.QueryRow(query, args...).Scan(&sCals)
	if err != nil {
		return nil, err
	}
	cals, err := deserializeCalories(sCals)
	if err != nil {
		return nil, err
	}

	return cals, nil
}

func (c *CaloriesData) Update(date time.Time, cals []int) error {
	query := `
		UPDATE calories
		SET calories = ?1
		WHERE date = ?2`
	args := []any{serializeCalories(cals), date.Format(time.DateOnly)}

	_, err := c.db.Exec(query, args...)
	if err != nil {
		return err
	}

	return nil
}

func (c *CaloriesData) Delete(date time.Time) error {
	query := `
		DELETE FROM calories
		WHERE date = ?1`
	args := []any{date.Format(time.DateOnly)}

	_, err := c.db.Exec(query, args...)
	if err != nil {
		return err
	}

	return nil
}

func (c *CaloriesData) Sum(date time.Time) (int, error) {
	cals, err := c.Read(date)
	if err != nil {
		return 0, err
	}

	sum := 0
	for _, val := range cals {
		sum += val
	}

	return sum, nil
}

func (c *CaloriesData) SafeDeleteLastElement(date time.Time) error {
	calls, err := c.Read(date)
	if err != nil {
		switch err {
		case sql.ErrNoRows:
			return nil
		default:
			return err
		}
	}

	if len(calls) == 0 {
		return nil
	}

	if len(calls) == 1 {
		err := c.Delete(date)
		if err != nil {
			return err
		}
		return nil
	}

	err = c.Update(date, calls[:len(calls)-1])
	if err != nil {
		return err
	}

	return nil
}

func (c *CaloriesData) Fill(date time.Time, fillTo int) error {
	calsSum, err := c.Sum(date)
	if err != nil {
		switch err {
		case sql.ErrNoRows:
			err := c.Create(date, []int{fillTo})
			if err != nil {
				return err
			}
			return nil
		default:
			return err
		}
	}

	if calsSum == fillTo {
		return nil
	}
	if calsSum > fillTo {
		return errors.New("can't fill calories to value bigger than current sum")
	}

	err = c.add(date, []int{fillTo - calsSum})
	if err != nil {
		return err
	}

	return nil
}

func (c *CaloriesData) Stats(date time.Time, targetCals int) (string, error) {
	cals, err := c.Read(date)
	if err != nil {
		switch err {
		case sql.ErrNoRows:
			cals = make([]int, 0)
		default:
			return "", err
		}
	}
	sum, err := c.Sum(date)
	if err != nil {
		switch err {
		case sql.ErrNoRows:
			sum = 0
		default:
			return "", err
		}
	}

	return fmt.Sprintf("list: %v\nsum: %d\nleft: %d (target: %d)", cals, sum, targetCals-sum, targetCals), nil
}

func (c *CaloriesData) add(date time.Time, cals []int) error {
	newCalls, err := c.Read(date)
	if err != nil {
		return err
	}
	newCalls = append(newCalls, cals...)

	err = c.Update(date, newCalls)
	if err != nil {
		return err
	}

	return nil
}

func serializeCalories(cals []int) string {
	sCals := make([]string, len(cals))
	for i, cal := range cals {
		sCals[i] = strconv.Itoa(cal)
	}
	return strings.Join(sCals, ",")
}

func deserializeCalories(sCals string) ([]int, error) {
	split := strings.Split(sCals, ",")
	cals := make([]int, len(split))
	for i, cal := range split {
		parsed, err := strconv.Atoi(cal)
		if err != nil {
			return nil, err
		}
		cals[i] = parsed
	}
	return cals, nil
}
