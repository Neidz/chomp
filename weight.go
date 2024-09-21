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

func (w *Weight) Delete(date time.Time) error {
	delete(w.Data, date)
	return w.save()
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
		return fmt.Sprintf("weight: -\n")
	}

	return fmt.Sprintf("[%s]\nweight: %.1f\n", date.Format(time.DateOnly), weight)
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
	weight.save()

	return nil
}
