.PHONY: all day_01 day_02 day_03 day_04

all: day_01 day_02 day_03 day_04

day_01:
	@echo "\nDay 1: Trebuchet?!"
	@cargo run -q --manifest-path=day_01/Cargo.toml

day_02:
	@echo "\nDay 2: Cube Conundrum"
	@(cd ./day_02/ && zig build run)

day_03:
	@echo "\nDay 3: Gear Ratios"
	@(cd ./day_03 && go run main.go)

day_04:
	@echo "\nDay 4: Scratchcards"
	@(cd ./day_04 && pkgx dotnet fsi ./main.fsx)
