package main

import (
	"fmt"

	"github.com/charmbracelet/lipgloss"
)

const ColorPrimary = lipgloss.Color("#8564fc")
const ColorAccent = lipgloss.Color("#a992fc")
const ColorText = lipgloss.Color("#C6D0F5")
const ColorError = lipgloss.Color("#C40F30")

var StylePrimary = lipgloss.NewStyle().Foreground(ColorPrimary)
var StyleTitle = lipgloss.NewStyle().Foreground(ColorAccent).Bold(true)
var StyleText = lipgloss.NewStyle().Foreground(ColorText)
var StyleError = lipgloss.NewStyle().Foreground(ColorError)

var StyleSelected = lipgloss.NewStyle().
	Foreground(ColorPrimary).
	BorderLeft(true).
	BorderStyle(lipgloss.NormalBorder()).
	BorderForeground(ColorPrimary)
var StyleOption = lipgloss.NewStyle().
	Foreground(ColorText).PaddingLeft(1)

func formattedForm(form Form) string {
	if !form.Active {
		return ""
	}

	s := ""
	s += StyleSelected.Bold(true).Render(fmt.Sprintf("%s ", form.Title))
	s += lipgloss.NewStyle().
		Foreground(ColorPrimary).Render(fmt.Sprintf("(%s)", form.Description))
	s += "\n"
	s += StyleSelected.Render(fmt.Sprintf("> %s", form.RawValue))
	s += "\n\n"

	return s
}

func formattedOptions(options []string, selectedOptionIndex int, markSelected bool) string {
	s := ""
	for i, option := range options {
		formattedOption := StyleOption.Render(option)
		if i == selectedOptionIndex && markSelected {
			formattedOption = StyleSelected.Render(option)
		}
		s += fmt.Sprintf("%s\n", formattedOption)
	}
	s += "\n"

	return s
}
