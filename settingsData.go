package main

import (
	"database/sql"
	"strconv"
)

type SettingsData struct {
	db *sql.DB
}

func (s *SettingsData) ReadTargetCalories() (int, error) {
	sCals, err := s.read("target_calories")
	if err != nil {
		return 0, err
	}

	targetCals, err := strconv.Atoi(sCals)
	if err != nil {
		return 0, err
	}

	return targetCals, nil
}

func (s *SettingsData) UpdateTargetCalories(targetCals int) error {
	sTargetCals := strconv.Itoa(targetCals)
	err := s.update("target_calories", sTargetCals)
	if err != nil {
		return err
	}

	return nil
}

func (s *SettingsData) read(key string) (string, error) {
	query := `
		SELECT value
		FROM settings
		WHERE key = ?1`
	args := []any{key}

	var value string
	err := s.db.QueryRow(query, args...).Scan(&value)
	if err != nil {
		return "", err
	}

	return value, nil
}

func (s *SettingsData) update(key, value string) error {
	query := `
		UPDATE settings
		SET value = ?1
		WHERE key = ?2`
	args := []any{value, key}

	_, err := s.db.Exec(query, args...)
	if err != nil {
		return err
	}

	return nil
}
