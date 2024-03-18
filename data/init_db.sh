#!/usr/bin/env bash

dir="$(dirname "${0}")"

sqlite3 "$dir/ShoppingList.db" < "$dir/create_table.sql"
