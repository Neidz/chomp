package main

import (
	"context"
	"database/sql"
	"embed"
	"errors"
	"os"
	"path/filepath"
	"runtime"
	"time"

	"github.com/golang-migrate/migrate/v4"
	"github.com/golang-migrate/migrate/v4/database/sqlite3"
	"github.com/golang-migrate/migrate/v4/source/iofs"
	_ "github.com/mattn/go-sqlite3"
)

//go:embed migrations/*
var migrations embed.FS

func PrepareDb() (*sql.DB, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return nil, errors.New("failed to get user's home directory")
	}

	var dbSource string
	if runtime.GOOS == "windows" {
		dbSource = filepath.Join(homeDir, "AppData", "Roaming", "chomp", "data.db")
	} else {
		dbSource = filepath.Join(homeDir, ".local", "share", "chomp", "data.db")
	}

	migrationsSource, err := iofs.New(migrations, "migrations")
	if err != nil {
		return nil, err
	}

	db, err := sql.Open("sqlite3", dbSource)
	if err != nil {
		return nil, err
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	err = db.PingContext(ctx)
	if err != nil {
		db.Close()
		return nil, err
	}

	driver, err := sqlite3.WithInstance(db, &sqlite3.Config{})
	if err != nil {
		return nil, err
	}

	m, err := migrate.NewWithInstance(
		"iofs", migrationsSource, "sqlite3", driver)
	if err != nil {
		return nil, err
	}

	err = m.Up()
	if err != nil && !errors.Is(err, migrate.ErrNoChange) {
		return nil, err
	}

	return db, nil
}
