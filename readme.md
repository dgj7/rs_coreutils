# WC Rewrite in Rust
Simple clone of the `wc` util, for learning purposes only.

Do not use this `rwc` program in lieu of `wc` under any circumstances.  This is not a suitable replacement.

Note that, as of `v1.0.2`, `rwc` doesn't yet return the same values as `wc`.

---
from `man wc`:
```
NAME
       wc - print newline, word, and byte counts for each file

SYNOPSIS
       wc [OPTION]... [FILE]...
       wc [OPTION]... --files0-from=F

DESCRIPTION
       Print  newline,  word,  and  byte  counts  for  each  FILE, and a total line if more than one FILE is specified.  A word is a
       non-zero-length sequence of characters delimited by white space.

       With no FILE, or when FILE is -, read standard input.

       The options below may be used to select which counts are printed, always in the following order:  newline,  word,  character,
       byte, maximum line length.

       -c, --bytes
              print the byte counts

       -m, --chars
              print the character counts

       -l, --lines
              print the newline counts

       --files0-from=F
              read input from the files specified by NUL-terminated names in file F; If F is - then read names from standard input

       -L, --max-line-length
              print the maximum display width

       -w, --words
              print the word counts

       --help display this help and exit

       --version
              output version information and exit

AUTHOR
       Written by Paul Rubin and David MacKenzie.

REPORTING BUGS
       GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
       Report any translation bugs to <https://translationproject.org/team/>

COPYRIGHT
       Copyright  ©  2020  Free  Software  Foundation,  Inc.   License  GPLv3+:  GNU  GPL  version  3  or later <https://gnu.org/li‐
       censes/gpl.html>.
       This is free software: you are free to change and redistribute it.  There is NO WARRANTY, to the extent permitted by law.

SEE ALSO
       Full documentation <https://www.gnu.org/software/coreutils/wc>
       or available locally via: info '(coreutils) wc invocation'
```

---
