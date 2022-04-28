% EA(1) Version 0.1.0

NAME
----

`ea` -- Making paths in CLI output actionable.

SYNOPSIS
--------

**ea** [_options_] **run** _style_ _executable_ [\-\- _argument_...]  
**ea** [_options_] [**list**]  
**ea** [_options_] **p[rint]** [_format_]

DESCRIPTION
-----------

Command-line tools often prints out file paths. Often, we want to follow up with some action on files in these paths, such as viewing, copying, editing, etc. When running these commands through `ea`, the paths in output will be stored in a local database, and become available for later use.

Use `ea run` to invoke your command. For example:

> **ea run** _grouped_ rg \-\- Vec src

... is how you run `rg Vec src` through `ea`. Note: any argument for `rg` comes after `--`.

_grouped_ hints at the format of `rg`'s output' so that `ea` knows how to find paths from `rg`'s output. This hint is necessary for `ea` to work with any many arbitrary commands as possible. See FORMATS to learn more.

You'll see that file locations in the original command are now prefixed with a number:

> src/something.rs  
> [1] 23: let list: Vec<String>  
> [2] 41: fn build() -> Vec<String>  
> ...

`ea list`, or simply `ea`, will print out these locations along with their numbers again:

> [1] src/something.rs:23
> [2] src/something.rs:41
> ...

With the numbers, `ea` can retrieve a corresponding path. In our example, `ea print` _2_ (or `ea p` _2_ in short) results in:
> src/something.rs

`ea print` takes an optional second argument _format_, making it possible to retrieve the location info mixed in this _format_ string. The sequences _{path}_, _{line}_, and _{column}_ appearing in _format_ get replaced by the location info. Running `ea p` _2_ _'{path} @ {line}'_  results in

> src/something.rs @ 41

`ea print`'s output is expected to be used as part of a longer command, such as `vim $(ea p 2)`.

It is recommended to create shell aliases or functions for frequently used `ea` commands.

FORMATS
-------

`ea run`'s first argument _style_ is mandatory. It indicates how file locations will appear in the output. This argument must be one of `grouped`, `linear`, or `search`. This section will explain what each of them means. The command you run through `ea` should have a matching _format_ value.

**grouped** indicates the command's output contains one or more sections, each section begins with a file path, and lines of line/column, and possibly more content, follows. An example of this _style_ of output is `ripgrep`'s default output:

> src/archive.rs  
> 41:pub fn read() -> Vec<Location> {  
> 45:pub fn read_from(path: &Path) -> Vec<Location> {  
>  
> src/interface.rs  
> 53:        arguments: Vec<String>,  

**linear** indicates each line in the command's output is a location. A location can be a file path, optionally followed by line, and, optionally, column number separated by ":" (colon). This is the default output from commands such as _find_ or _fd_:

> src/archive.rs:41  
> src/archive.rs:45  
> src/interface.rs:53  


**search** means the output is almost arbitrary, except every now and then, an location appears at the beginning of a line. This is common in error messages from compilers like _clang_ or _swift_: 

> Sources/Critic/DocProblem.swift:5:26: error: cannot find type 'Stringx' in scope  
>    public let filePath: Stringx

If you need `ea` to support more formats, please file an issue at https://github.com/dduan/ea

AUTHOR
------

Daniel Duan <daniel@duan.ca>
