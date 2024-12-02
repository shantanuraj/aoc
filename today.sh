#!/bin/zsh

# Get the current year and day
year=$(date +%Y)
day=$(date +%d)

# Create the directory structure
mkdir -p "$year/day_$day/src"

# Create a basic main.rs file
cat <<EOL > "$year/day_$day/src/main.rs"
fn main() {
    println!("Hello, Day $day!");
}
EOL

# Create a basic Cargo.toml file
cat <<EOL > "$year/day_$day/Cargo.toml"
[package]
name = "day_$day"
version = "0.1.0"
edition = "2021"

[dependencies]
EOL

# Create input_01.txt and input_02.txt files
touch "$year/day_$day/input_01.txt"
touch "$year/day_$day/input_02.txt"

# Update the Makefile
makefile="$year/Makefile"
if ! grep -q "day_$day" "$makefile"; then
    echo -e "\n.PHONY: day_$day\nday_$day:\n\t@echo \"Day $day: New Challenge\"\n\t@cargo run -q --manifest-path=day_$day/Cargo.toml" >> "$makefile"
fi

# Echo the make command to run the new package
echo "make -C $year day_$day"
