# watcher

how to use:

```bash
$ watcher -h
```

dev info (3 args):
- 1: path to file to watch
- 2: destination path for where the parser crate is to be generated
- 3: desired generated parser crate name

example usage:
```bash
$ watcher src/grammar.pp src/my_spicy_grammar_parser my_spicy_parser
```
