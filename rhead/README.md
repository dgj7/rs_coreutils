# rhead
`head`, rewritten in `rust` for learning purposes.

Do not use this `hread` program in lieu of `head` under any circumstances.  This is not a suitable replacement.

## Synopsis
From `head --help` on `Debian`:
```bash
Print the first 10 lines of each FILE to standard output.
With more than one FILE, precede each with a header giving the file name.

With no FILE, or when FILE is -, read standard input.
```

For more information, see `head --help`, `man head` or [man-pages](https://man7.org/linux/man-pages/man1/head.1.html).

## Progress
| Status             | Feature                                                                                                                                                                                                                                     |
|--------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| :heavy_check_mark: | initialize project                                                                                                                                                                                                                          |
| :x:                | argument parsing, with config struct                                                                                                                                                                                                        |
| :x:                | _core_ `head` functionality implemented                                                                                                                                                                                                     |
| :x:                | fix warnings (clippy)                                                                                                                                                                                                                       |
| :x:                | full unit test suite                                                                                                                                                                                                                        |
| :x:                | additional functionality: NUM may have a multiplier suffix: b 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024, GB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y. Binary prefixes can be used, too: KiB=K, MiB=M, and so on. |

| Status | Parameter             | Detail                                                                                                                 |
|--------|-----------------------|------------------------------------------------------------------------------------------------------------------------|
| :x:    | -c, --bytes=[-]NUM    | print the first NUM bytes of each file; with the leading '-', print all but the last NUM bytes of each file            |            
| :x:    | -n, --lines=[-]NUM    | print the first NUM lines instead of the first 10; with the leading '-', print all but the last NUM lines of each file | 
| :x:    | -q, --quiet, --silent | never print headers giving file names                                                                                  |                                                                                  
| :x:    | -v, --verbose         | always print headers giving file names                                                                                 |                                                                                 
| :x:    | -z, --zero-terminated | line delimiter is NUL, not newline                                                                                     |                                                                                     
| :x:    | --help                | display this help and exit                                                                                             |                                                                                             
| :x:    | --version             | output version information and exit                                                                                    |

---
[Go Back](..)
