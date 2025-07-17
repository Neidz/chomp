# Chomp
Welcome to Chomp, application for tracking calories, fats, proteins, carbohydrates and weight. With Chomp you create your own library of products and then construct meals with them, Chomp helps you track calories and macros for those meals and also track your weight so that you can understand your progress. Everything is stored locally, there's no account and no need for internet.

## Showcase
<img width="2551" height="1434" alt="dashboard" src="https://github.com/user-attachments/assets/89fb9339-5cfa-4a8b-8734-138cd7595af9" />
<img width="2554" height="1435" alt="meals" src="https://github.com/user-attachments/assets/64f44836-921a-43be-9016-fc9f8d8bed2a" />
<img width="2555" height="1430" alt="meals add product" src="https://github.com/user-attachments/assets/44dafa35-ba0e-4c91-95a0-32b7d9b3e20b" />
<img width="2552" height="1434" alt="products" src="https://github.com/user-attachments/assets/e4eea21f-9aea-463b-99a2-968f7fe9d7c9" />
<img width="2553" height="1434" alt="weights" src="https://github.com/user-attachments/assets/e7906ec6-811c-43da-ab72-5496cc940733" />
<img width="2551" height="1430" alt="targets" src="https://github.com/user-attachments/assets/18fe52b8-0f1c-4cf4-b4ed-e1227d5d07f3" />

https://github.com/user-attachments/assets/ee579f7b-4d1b-45a1-b71d-d18a9d88533f

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
