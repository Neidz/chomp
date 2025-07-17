# Chomp
Welcome to Chomp, application for tracking calories, fats, proteins, carbohydrates and weight. With Chomp you create your own library of products and then construct meals with them, Chomp helps you track calories and macros for those meals and also track your weight so that you can understand your progress. Everything is stored locally, there's no account and no need for internet.
![chomp_showcase](https://github.com/user-attachments/assets/210f525e-6d37-4462-a3b2-424cf42cfea5)

## Development and stability
Chomp is still in active development and I have some plans for new features that will be added in the future. That being said it can be considered stable and it won't break your library of products and history of meals/weight. You will never have to remove old database and create new one because of some breaking changes.

## Installation
#### Linux
- clone the repository

```bash
git clone https://github.com/Neidz/chomp.git
```

- enter project directory

```bash
cd chomp
```

- run installation script

```bash
script/install-linux.sh
```

#### MacOS
You should be able to build and run Chomp on MacOS without any issues but installation script for linux won't work because MacOS handles desktop entries differently, requires signing apps and so on. I'm not going to deal with this process since I'm not using MacOS but feel free to install binary with

```bash
cargo install --path ./chomp-app
```

#### Windows
I never ran this app on windows and I'm not even sure if path for database will be properly constructed but if you would like to actually use Chomp on windows let me know and I'll test/fix this if there's an issue with that. Not going to deal with creating proper installer tho
