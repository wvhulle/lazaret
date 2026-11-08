PREFIX ?= /usr/local
DESTDIR ?=
CARGO ?= cargo

APPLICATION_ID := page.codeberg.wvhulle.Lazaret
RELEASE_BINARY := target/release/lazaret
RELEASE_ADMIN_SERVICE_BINARY := target/release/lazaret-clamav-admin
ADMIN_BUS_NAME := $(APPLICATION_ID).ClamavAdmin
SYSCONFDIR ?= /etc
COMPILE_SCHEMAS ?= yes
INSTALL_ROOT := $(DESTDIR)$(PREFIX)
SHARE_DIRECTORY := $(INSTALL_ROOT)/share
ADMIN_SERVICE_DIRECTORY := $(PREFIX)/libexec/lazaret
CONFIG_ROOT := $(DESTDIR)$(SYSCONFDIR)

.PHONY: build install install-binary install-data install-metainfo install-clamav-admin install-user-service install-desktop-entry install-icons install-settings-schema

build:
	$(CARGO) build --release --locked --package lazaret --package lazaret-clamav-admin

install: install-binary install-data install-clamav-admin

install-binary:
	install -Dm755 $(RELEASE_BINARY) $(INSTALL_ROOT)/bin/lazaret

install-data: install-desktop-entry install-metainfo install-icons install-settings-schema install-user-service

install-metainfo:
	install -Dm644 data/desktop_integration/$(APPLICATION_ID).metainfo.xml.in $(SHARE_DIRECTORY)/metainfo/$(APPLICATION_ID).metainfo.xml

install-clamav-admin:
	install -Dm755 $(RELEASE_ADMIN_SERVICE_BINARY) $(DESTDIR)$(ADMIN_SERVICE_DIRECTORY)/lazaret-clamav-admin
	install -Dm644 clamav_admin_service/system_files/$(APPLICATION_ID).policy $(SHARE_DIRECTORY)/polkit-1/actions/$(APPLICATION_ID).policy
	install -Dm644 clamav_admin_service/system_files/$(APPLICATION_ID).ClamavAdmin.conf $(SHARE_DIRECTORY)/dbus-1/system.d/$(ADMIN_BUS_NAME).conf
	install -Dm644 clamav_admin_service/system_files/$(APPLICATION_ID).ClamavAdmin.service $(SHARE_DIRECTORY)/dbus-1/system-services/$(ADMIN_BUS_NAME).service
	install -d $(INSTALL_ROOT)/lib/systemd/system
	sed 's|@ADMIN_SERVICE_PATH@|$(ADMIN_SERVICE_DIRECTORY)/lazaret-clamav-admin|' clamav_admin_service/system_files/lazaret-clamav-admin.service.in > $(INSTALL_ROOT)/lib/systemd/system/lazaret-clamav-admin.service
	sed 's|@ADMIN_SERVICE_PATH@|$(ADMIN_SERVICE_DIRECTORY)/lazaret-clamav-admin|' clamav_admin_service/system_files/lazaret-settings-merge.service.in > $(INSTALL_ROOT)/lib/systemd/system/lazaret-settings-merge.service
	sed 's|@ADMIN_SERVICE_PATH@|$(ADMIN_SERVICE_DIRECTORY)/lazaret-clamav-admin|' clamav_admin_service/system_files/lazaret-reduction-expiry.service.in > $(INSTALL_ROOT)/lib/systemd/system/lazaret-reduction-expiry.service
	install -Dm644 clamav_admin_service/system_files/lazaret-reduction-expiry.timer $(INSTALL_ROOT)/lib/systemd/system/lazaret-reduction-expiry.timer
	install -Dm644 clamav_admin_service/system_files/clamav-daemon-live-tuning.conf $(SHARE_DIRECTORY)/lazaret/clamav-daemon-live-tuning.conf
	install -Dm644 clamav_admin_service/system_files/clamav-clamonacc-live-tuning.conf $(SHARE_DIRECTORY)/lazaret/clamav-clamonacc-live-tuning.conf
	install -d $(CONFIG_ROOT)/lazaret
	test -e $(CONFIG_ROOT)/lazaret/tunable-settings || install -m644 /dev/null $(CONFIG_ROOT)/lazaret/tunable-settings

install-user-service:
	install -d $(INSTALL_ROOT)/lib/systemd/user
	sed 's|@BINARY_PATH@|$(PREFIX)/bin/lazaret|' data/user_units/lazaret.service.in > $(INSTALL_ROOT)/lib/systemd/user/lazaret.service

install-desktop-entry:
	install -Dm644 data/desktop_integration/$(APPLICATION_ID).desktop.in $(SHARE_DIRECTORY)/applications/$(APPLICATION_ID).desktop

install-icons:
	install -Dm644 data/desktop_integration/icons/hicolor/scalable/apps/$(APPLICATION_ID).svg $(SHARE_DIRECTORY)/icons/hicolor/scalable/apps/$(APPLICATION_ID).svg
	install -Dm644 data/desktop_integration/icons/hicolor/symbolic/apps/$(APPLICATION_ID)-symbolic.svg $(SHARE_DIRECTORY)/icons/hicolor/symbolic/apps/$(APPLICATION_ID)-symbolic.svg
	install -Dm644 data/desktop_integration/icons/hicolor/symbolic/apps/$(APPLICATION_ID)-attention-symbolic.svg $(SHARE_DIRECTORY)/icons/hicolor/symbolic/apps/$(APPLICATION_ID)-attention-symbolic.svg

install-settings-schema:
	install -Dm644 data/desktop_integration/$(APPLICATION_ID).gschema.xml $(SHARE_DIRECTORY)/glib-2.0/schemas/$(APPLICATION_ID).gschema.xml
	test "$(COMPILE_SCHEMAS)" != yes || glib-compile-schemas $(SHARE_DIRECTORY)/glib-2.0/schemas
