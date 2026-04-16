#!/bin/bash

if [ -z "$1" ]; then
    echo -e "\033[31m[ERROR]\033[0m No commit name!"
    echo "Usage: ./gitpush.sh \"Commit name\""
    exit 1
fi

OK="\033[32m[OK]\033[0m"
INFO="\033[34m[INFO]\033[0m"
ERROR="\033[31m[ERROR]\033[0m"

git status

read -p "Continue? (y/n): " confirm
if [ "$confirm" != "y" ]; then
    echo -e "$INFO Stop."
    exit 0
fi

git add .

git commit -m "$1"
if [ $? -ne 0 ]; then
    echo -e "$ERROR Commit error!"
    exit 1
fi

# Пушим
echo -e "$INFO git push -u origin main"
git push -u origin main
if [ $? -ne 0 ]; then
    echo -e "$ERROR Push error!"
    exit 1
fi

echo -e "$OK Готово!"
