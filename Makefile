tests:
	@echo "clear comm!"
	rm -rf comm/src/test/*
	@echo "clear db!"
	rm -rf db/src/test/*
	@echo "test all!"
	cargo test