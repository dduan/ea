This file shows examples of shell functions that consumes output of `ea print`. It is recommended to include them as part of your `ea` workflow.


# Editor Alias

This shell function enables typing `e NUM` in shell to open the path corrsponding to `NUM` in your default editor.

Find the shell/editor combination you need below!

|            | Bash                            | Zsh                            | Fish                            |
| ---------- | ------------------------------- | ------------------------------ | ------------------------------- |
| Vim/NeoVim | [link](#bash-and-vim-or-neovim) | [link](#zsh-and-vim-or-neovim) | [link](#fish-and-vim-or-neovim) |
| VSCode     | [link](#bash-and-vscode)        | [link](#zsh-and-vscode)        | [link](#fish-and-vscode)        |
| Emacs      | [link](#bash-and-emacs)         | [link](#zsh-and-emacs)         | [link](#fish-and-emacs)         |

## Bash and Vim or NeoVim

```bash
e() {
    eval $(ea p $1 "$EDITOR '{path}' '+call cursor({line}, {column})'")
}
```

## Zsh and Vim or NeoVim

```zsh
function e {
    eval $(ea p $1 "$EDITOR '{path}' '+call cursor({line}, {column})'")
}
```

## Fish and Vim or NeoVim

```fish
function e
  eval (ea p $argv '$EDITOR "{path}" "+call cursor({line}, {column})"')
end
```

## Bash and VSCode

```bash
e() {
    eval $(ea p $1 "code --goto '{path}:{line}:{column}'")
}
```

## Zsh and VSCode

```zsh
function e {
    eval $(ea p $1 "code --goto '{path}:{line}:{column}'")
}
```

## Fish and VSCode

```fish
function e
  eval (ea p $argv "code --goto '{path}:{line}:{column}'")
end
```

## Bash and Emacs

```bash
e() {
    eval $(ea p $1 "emacs +{line}:{column} '{path}'")
}
```

## Zsh and Emacs

```zsh
function e {
    eval $(ea p $1 "emacs +{line}:{column} '{path}'")
}
```

## Fish and Emacs

```fish
function e
    eval (ea p 20 "emacs +{line}:{column} '{path}'")
end
```
