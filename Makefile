DAY = 70

all: $(DAY).rs
	rustc $(DAY).rs
	./$(DAY)

prep: template.rs
	cp template.rs $(DAY).rs
