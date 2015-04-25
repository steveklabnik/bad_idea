default:
	cargo build --release
	cp target/release/libbad_idea-*.a ext/bad_idea/libbad_idea.a
	cd ext/bad_idea; ruby extconf.rb; make clean; make
	rake build
	ruby -Ilib:ext -rbad_idea -e 'puts BadIdea::double_input(4)'
