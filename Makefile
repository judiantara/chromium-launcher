.PHONY: debug release clean run install

debug:
	@cargo build

release:
	@cargo build -r

clean:
	@cargo clean

run: debug
	target/debug/chromium-app-launcher whatsapp https://web.whatsapp.com

install: clean release
	@rm -f ~/.local/bin/chromium-app-launcher
	@cp target/release/chromium-app-launcher ~/.local/bin/
