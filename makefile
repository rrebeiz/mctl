APP_ID=com.github.rrebeiz.mctl
SERVICE_PATH=$(HOME)/.config/systemd/user/mctl.service

## build: builds the app
build:
	@echo "building..."
	cargo build --release
	@echo "built!"

## dev: runs dev mode
dev:
	@echo "running dev"
	cargo run

## install: installs binary + systemd service
install:
	@echo "installing binary..."
	cargo install --path . --force

	@echo "creating systemd service..."
	mkdir -p $(HOME)/.config/systemd/user

	cp mctl.service $(SERVICE_PATH)
	systemctl --user daemon-reload
	systemctl --user enable --now mctl
	@echo "installed and started!"

## uninstall: uninstalls the service and binary
uninstall:
	@echo "stopping service..."
	systemctl --user disable --now mctl || true

	@echo "removing service file..."
	rm -f $(SERVICE_PATH)
	systemctl --user daemon-reload

	@echo "removing binary..."
	cargo uninstall mctl || true

	@echo "uninstalled!"

## install-bin: downloads the prebuilt binary and installs it
install-bin:
	@echo "downloading prebuilt binary..."
	curl -L https://github.com/rrebeiz/mctl/releases/latest/download/mctl -o mctl

	chmod +x mctl
	sudo install -Dm755 mctl /usr/local/bin/mctl
	rm mctl

	mkdir -p $(HOME)/.config/systemd/user
	cp mctl-bin.service $(SERVICE_PATH)

	systemctl --user daemon-reload
	systemctl --user enable --now mctl

## uninstall-bin: removes the binary and service file
uninstall-bin:
	@echo "stopping service..."
	systemctl --user disable --now mctl || true

	@echo "removing service file..."
	rm -f $(SERVICE_PATH)
	systemctl --user daemon-reload

	@echo "removing binary (requires sudo)..."
	sudo rm -f /usr/local/bin/mctl

	@echo "uninstall complete!"
