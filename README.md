# ea: Editor Alias

`ea` remembers file paths from CLI output for later use.

## Intro

CLI programs like `find`, `rg`, `clang` etc, print out file paths (and line/columns). `ea` lets you act on
them. Here's how:

```
# Run your command through `ea`. We'll use `fd '\.rs'` as example:
$ ea run linear fd -- '\.rs$'
[1] src/archive.rs
[2] src/commands.rs
[3] src/interface.rs
[4] src/lib.rs
[5] src/parsers.rs
[6] tests/test_parsers.rs

# Use the number added by `ea` to retrieve the path info.
# Let's open [5] src/parsers.rs with vim
$ vim $(ea p 5)
```

By running your command through `ea`, each path in its output get marked with a number in the front. This
number can be used to retrieve its corresponding path later for your purposes. Combined with some shell
configurations, this can provide a powerful experience.

## Shell integration

First, you'll want alias your normal command to the `ea` version:

```fish
alias fd 'ea run linear fd --' # more explanation on this syntax later
```

Then, optionally, make a shell function that consume a path from `ea`'s output.

```fish
# calling `e N` makes nvim opens Nth location stored by `ea`.
function e
    eval (ea print $argv 'nvim \"{path}\" \"+call cursor({line}, {column})\"')
end
```

## Subcommands, explained

`ea` provides 3 subcommands that compliment each other.

### `ea run`

Run a command, add annotation to paths included in its output, and store each path for later use. `ea` expects
a hint for your command's output format. It must be one of the following:

1. linear
2. grouped
3. search

The specifics of these formats is explained in [formats][].

For example, `ripgrep`'s default output format matches the `grouped` style, so you'd run:

```
ea run grouped rg ...
```

All command line arguments for the actual command must follow `--`. Furthering our example, this is how to run
`rg Vec` with `ea`:

```
ea run grouped rg -- Vec
```

### `ea print`

Alternatiely, `ea p`. This command prints out a stored location. You tell it which location number to print,
and, optionally, what format to print it in. First example, `ea p 3` will print out the 3rd location. And
`ea print 1 '{path} L{line} C{column}'` might print something like `a/b/c.swift L23 C15`. The second argument
is a format string where `{path}` / `{line}` / `{column}` get replaced by corresponding values.


### `ea list`

Or simply `ea`. This subcommand prints out a list of stored locations with their associated numbers.

## LICENSE

See [LICENSE.txt]
