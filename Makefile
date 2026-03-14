.PHONY: man
man:
	sed -i ':a;N;$$!ba;s/default: \n/default: \\\\n/' man/prefix.1

.PHONY: install-fish
install-fish:
	install -m 644 completion/prefix.fish "$(HOME)/.config/fish/completions/prefix.fish"

.PHONY: install-zsh
install-zsh:
	install -m 644 completion/_prefix /usr/share/zsh/functions/Completion/_prefix

.PHONY: install-bash
install-bash:
	install -m 644 completion/_prefix /etc/bash_completion.d/prefix.bash

.PHONY: install-man
install-man:
	install -m 644 man/prefix.1 /usr/share/man/man1/prefix.1
