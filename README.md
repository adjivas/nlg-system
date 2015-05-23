NLG-System
==========

[![Build Status](https://travis-ci.org/adjivas/nlg-system.svg)](https://travis-ci.org/adjivas/nlg-system)
[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)

This library parses all word by type from a list of file. All types are separated by "dictionary", a word into a dictionary has a index.

#### Example:
```shell
$ cargo run
1-5
executed 
1-6
generated 
1-7
previewing 
4-5
an 
4-6
that 
4-7
than 
4-8
or
...
```

#### Directory-Tree:

```shell
.
|_ Cargo.toml
|_ data
|   \_ man.word
|_ LICENSE
|_ README.md
\_ src
    |_ bin.rs
    \_ lib.rs
```

# License
*nlg-system*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/nlg-system/blob/master/LICENSE).
