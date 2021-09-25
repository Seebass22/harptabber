WINDOWS = x86_64-pc-windows-gnu
OUTDIR = builds

build: windows linux web

windows:
	cargo build --release -p harptabber --target $(WINDOWS)
	cargo build --release -p harptabber-gui --target $(WINDOWS)

linux:
	cargo build --release -p harptabber
	cargo build --release -p harptabber-gui

web:
	./build_web.sh

pack:
	rm -f *.zip
	rm -rf $(OUTDIR)
	mkdir -p $(OUTDIR)
	zip -r $(OUTDIR)/web.zip docs
	cp target/release/harptabber-gui target/release/harptabber target/$(WINDOWS)/release/harptabber.exe target/$(WINDOWS)/release/harptabber-gui.exe $(OUTDIR)

itch: pack
	butler push $(OUTDIR)/web.zip seebass22/harmonica-tab-transposer:html5
	butler push $(OUTDIR)/harptabber-gui.exe seebass22/harmonica-tab-transposer:windows
	butler push $(OUTDIR)/harptabber.exe seebass22/harmonica-tab-transposer:windows-cli
	butler push $(OUTDIR)/harptabber-gui seebass22/harmonica-tab-transposer:linux
	butler push $(OUTDIR)/harptabber seebass22/harmonica-tab-transposer:linux-cli
