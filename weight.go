package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"time"
)

type WeightData map[time.Time]float32

func (w WeightData) MarshalJSON() ([]byte, error) {
	tmp := make(map[string]float32, len(w))
	for k, v := range w {
		tmp[k.Format(time.DateOnly)] = v
	}

	return json.Marshal(tmp)
}

func (w *WeightData) UnmarshalJSON(data []byte) error {
	tmp := make(map[string]float32)
	if err := json.Unmarshal(data, &tmp); err != nil {
		return err
	}

	parsed := make(WeightData)
	for k, v := range tmp {
		t, err := time.Parse(time.DateOnly, k)
		if err != nil {
			return err
		}
		parsed[t] = v
	}
	*w = parsed

	return nil
}

type Weight struct {
	Path    string     `json:"-"`
	Data    WeightData `json:"data"`
	Version int32      `json:"version"`
}

func (w *Weight) Get(date time.Time) float32 {
	weight, ok := w.Data[date]
	if !ok {
		return 0
	}

	return weight
}

func (w *Weight) Set(date time.Time, weight float32) error {
	w.Data[date] = weight
	return w.save()
}

func (w *Weight) SafeSet(date time.Time, weight float32) error {
	_, ok := w.Data[date]
	if ok {
		return ErrDateRecordAlreadyExists
	} else {
		w.Data[date] = weight
	}

	return w.save()
}

func (w *Weight) Delete(date time.Time) error {
	delete(w.Data, date)
	return w.save()
}

func (w *Weight) averageFromRange(startDate time.Time, endDate time.Time) (float32, error) {
	switch endDate.Compare(startDate) {
	case 0:
		return 0, fmt.Errorf("invalid range, endDate has to be different than startDate")
	case -1:
		return 0, fmt.Errorf("invalid range, endDate has to be date after startDate")
	}

	amount := float32(0)
	sum := 0

	for d := startDate; !d.After(endDate); d = d.AddDate(0, 0, 1) {
		weight, ok := w.Data[d]
		if ok {
			amount += weight
			sum++
		}
	}

	if sum == 0 {
		return 0, fmt.Errorf("no entires for this range")
	}

	return amount / float32(sum), nil
}

func (w *Weight) weeklyChange() string {
	today := Today()
	avgLastWeek, err := w.averageFromRange(today.AddDate(0, 0, -7), today)
	if err != nil {
		return "-"
	}
	avgWeekBefore, err := w.averageFromRange(today.AddDate(0, 0, -14), today.AddDate(0, 0, -7))
	if err != nil {
		return "-"
	}
	weeklyChange := avgLastWeek - avgWeekBefore

	return fmt.Sprintf("%.1f", weeklyChange)
}

func (w *Weight) save() error {
	bytes, err := json.MarshalIndent(w, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to marshal weight data to json: %w", err)
	}

	err = os.WriteFile(w.Path, bytes, 0644)
	if err != nil {
		return fmt.Errorf("failed to save weight data to file: %w", err)
	}

	return nil
}

func (w *Weight) Stats(date time.Time) string {
	weight, ok := w.Data[date]
	if !ok {
		return fmt.Sprintf("[%s]\nweight: -\n", date)
	}

	weeklyChange := w.weeklyChange()

	return fmt.Sprintf("[%s]\nweight: %.1f\nweekly change: %s\n", date.Format(time.DateOnly), weight, weeklyChange)
}

func LoadWeight(path string) (Weight, error) {
	_, err := os.Stat(path)
	if errors.Is(err, os.ErrNotExist) {
		err = createDefaultWeight(path)
		if err != nil {
			return Weight{}, fmt.Errorf("failed to create default weight data: %w", err)
		}
		fmt.Println("weight data not found, creating default")
	} else if err != nil {
		return Weight{}, err
	}

	data, err := openWeight(path)
	if err != nil {
		return Weight{}, fmt.Errorf("failed to open weight data: %w", err)
	}

	data.Path = path

	return data, nil
}

func openWeight(path string) (Weight, error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return Weight{}, err
	}

	var weight Weight
	err = json.Unmarshal(bytes, &weight)
	if err != nil {
		return Weight{}, err
	}

	return weight, nil
}

func createDefaultWeight(path string) error {
	err := os.MkdirAll(filepath.Dir(path), 0755)
	if err != nil {
		return err
	}

	weight := Weight{
		Data: make(map[time.Time]float32),
		Path: path,
	}
	return weight.save()
}
