package main

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"time"
)

type CaloriesData map[time.Time][]int

func (c CaloriesData) MarshalJSON() ([]byte, error) {
	tmp := make(map[string][]int, len(c))
	for k, v := range c {
		tmp[k.Format(time.DateOnly)] = v
	}

	return json.Marshal(tmp)
}

func (c *CaloriesData) UnmarshalJSON(data []byte) error {
	tmp := make(map[string][]int)
	if err := json.Unmarshal(data, &tmp); err != nil {
		return err
	}

	parsed := make(CaloriesData)
	for k, v := range tmp {
		t, err := time.Parse(time.DateOnly, k)
		if err != nil {
			return err
		}
		parsed[t] = v
	}
	*c = parsed

	return nil
}

type Calories struct {
	Path           string       `json:"-"`
	Data           CaloriesData `json:"data"`
	TargetCalories int          `json:"target_calories"`
	Version        int          `json:"version"`
}

func (c *Calories) Add(date time.Time, toAdd []int) error {
	calories, ok := c.Data[date]
	if ok {
		c.Data[date] = append(calories, toAdd...)
	} else {
		c.Data[date] = toAdd
	}

	return c.save()
}

func (c *Calories) SafeAdd(date time.Time, toAdd []int) error {
	_, ok := c.Data[date]
	if ok {
		return ErrDateRecordAlreadyExists
	} else {
		c.Data[date] = toAdd
	}

	return c.save()
}

func (c *Calories) Sum(date time.Time) int {
	calories, ok := c.Data[date]
	if !ok {
		return 0
	}

	sum := 0
	for _, val := range calories {
		sum += val
	}

	return sum
}

func (c *Calories) List(date time.Time) []int {
	calories, ok := c.Data[date]
	if !ok {
		return make([]int, 0)
	}

	return calories
}

func (c *Calories) Delete(date time.Time) error {
	delete(c.Data, date)
	return c.save()
}

func (c *Calories) Pop(date time.Time) error {
	calories, ok := c.Data[date]
	if !ok || len(calories) == 0 {
		return nil
	}

	c.Data[date] = calories[:len(calories)-1]
	return c.save()
}

func (c *Calories) Fill(date time.Time) error {
	calories, ok := c.Data[date]
	if !ok {
		c.Data[date] = []int{}
	}

	sum := c.Sum(date)

	if sum == c.TargetCalories {
		return nil
	}
	if sum > c.TargetCalories {
		return fmt.Errorf("can't fill calories to value bigger than current sum")
	}

	c.Data[date] = append(calories, c.TargetCalories-sum)
	return c.save()
}

func (c *Calories) GetTarget() int {
	return c.TargetCalories
}

func (c *Calories) SetTarget(target int) error {
	c.TargetCalories = target
	return c.save()
}

func (c *Calories) save() error {
	bytes, err := json.MarshalIndent(c, "", "  ")
	if err != nil {
		return fmt.Errorf("failed to marshal calories data to json: %w", err)
	}

	err = os.WriteFile(c.Path, bytes, 0644)
	if err != nil {
		return fmt.Errorf("failed to save calories data to file: %w", err)
	}

	return nil
}

func (c *Calories) Stats(date time.Time) string {
	calories, ok := c.Data[date]
	if !ok {
		calories = make([]int, 0)
	}
	sum := c.Sum(date)

	return fmt.Sprintf("calories: %v\nsum: %d\nleft: %d (target: %d)\n", calories, sum, c.TargetCalories-sum, c.TargetCalories)
}

func LoadCalories(path string) (Calories, error) {
	_, err := os.Stat(path)
	if errors.Is(err, os.ErrNotExist) {
		err = createDefaultCalories(path)
		if err != nil {
			return Calories{}, fmt.Errorf("failed to create default calories data: %w", err)
		}
		fmt.Println("calories data not found, creating default")
	} else if err != nil {
		return Calories{}, err
	}

	data, err := openCalories(path)
	if err != nil {
		return Calories{}, fmt.Errorf("failed to open calories data: %w", err)
	}

	data.Path = path

	return data, nil
}

func openCalories(path string) (Calories, error) {
	bytes, err := os.ReadFile(path)
	if err != nil {
		return Calories{}, err
	}

	var calories Calories
	err = json.Unmarshal(bytes, &calories)
	if err != nil {
		return Calories{}, err
	}

	return calories, nil
}

func createDefaultCalories(path string) error {
	err := os.MkdirAll(filepath.Dir(path), 0755)
	if err != nil {
		return err
	}

	calories := Calories{
		Data:           make(map[time.Time][]int),
		Path:           path,
		TargetCalories: 0,
		Version:        1,
	}
	calories.save()

	return nil
}
