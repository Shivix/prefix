.PHONY: man
man:
	sed -i ':a;N;$$!ba;s/default: \n/default: \\\\n/' man/prefix.1

.PHONY: install-fish
install-fish:
	install -m 644 completion/prefix.fish /usr/share/fish/completions/prefix.fish
	install -m 644 man/prefix.1 /usr/share/man/man1/prefix.1

.PHONY: install-zsh
install-zsh:
	install -m 644 completion/_prefix /usr/share/zsh/functions/Completion/_prefix
	install -m 644 man/prefix.1 /usr/share/man/man1/prefix.1

.PHONY: install-bash
install-bash:
	install -m 644 completion/_prefix /etc/bash_completion.d/prefix.bash
	install -m 644 man/prefix.1 /usr/share/man/man1/prefix.1
