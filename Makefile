.PHONY: all 2023 2024

all: 2023 2024

2023:
	@make -C 2023

2024:
	@make -C 2024

.PHONY: today
today:
	@./today.sh
