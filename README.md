# Rustyline testing

This repo has a few tests which are used to validate behavior of the rustyline history buffer.

## Init file

```
$ cargo run --bin with-init-file
```

This test shows that when there are multiple duplicate lines in a history file
before rustyline opens it, it will truncate lines after editing the file.

The test writes this data to the history file with raw IO:

```
#V2
A --- long line
A --- long line

```

Next it creates a rustyline `Editor` and tells it to add the line `"B"` to the
history file. Finally it prints the contents of the history file so you can see
what it outputs.

Actual output:

```
#V2
A --- long line
B
--- long line
```

Expected output:

```
#V2
A --- long line
A --- long line
B
```

or

```
#V2
A --- long line
B
```

This should be fine, because rustyline should not write duplicate lines to the
history file. Unfortunately I have found that parallel writes can cause this to
happen.

## Parallel writes

```
$ cargo run --bin parallel-writes
```

This example shows that parallel writes can cause rustyline to duplicate lines,
which then leads to line truncation. This example uses no raw IO, it only uses
the rustyline library to write to the history file. It does _read_ from the file
using the rust stdlib but this is for convenience only and removing those reads
do not impact rustyline's behavior.

We have a function `run` which does the following:

- write some long lines to the history file
- write `ls` to the history file
- write some more long lines to the history file

We first run this function in parallel. Once on the main thread and once on a
background thread. At this point there should be some duplicate lines in the
file and we print out the contents of the file.

After this, we start a fresh rustyline editor, go through `run` again, and we
see the truncation bug show up.

Actual output (slightly random, depends on threading):

```
After parallel writes
#V2
.......... 0
.......... 1
.......... 1
ls
ls
.......... 0
.......... 0
.......... 1
.......... 1

After final writes
#V2
.......... 0
.......... 1
ls
.......... 0
.......... 1
.......... 0
 0
.......... 1
.......... 1
.......... 1
ls
.......... 0
.......... 1
```

Notice the line containing only ` 0`. This line was truncated. Notice that the
third `ls` does not show up.

Expected output (also slight randomization possible):

```
After parallel writes
#V2
.......... 0
.......... 0
.......... 1
.......... 1
ls
ls
.......... 0
.......... 0
.......... 1
.......... 1

After final writes
#V2
.......... 0
.......... 0
.......... 1
.......... 1
ls
ls
.......... 0
.......... 0
.......... 1
.......... 1
.......... 0
.......... 1
ls
.......... 0
.......... 1
```
