
all: attacker dumb-victim

attacker:
	rustc attacker.rs

dumb-victim:
	rustc dumb-victim.rs

clean:
	rm -f attacker dumb-victim
