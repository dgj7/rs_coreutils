# Coreutils - Rust
Rewriting coreutils with Rust.

Don't use any of these programs in lieu of the built-in commands; this project is for learning purposes only, and _is not a suitable replacement_.

## Text Processing And Input
| Utility | Status                                                                                         | Manual                                                                                          |
|---------|------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------|
| head    | [![rhead](https://github.com/dgj7/rs_coreutils/actions/workflows/rhead.yml/badge.svg)](/rhead) | [Displays the beginning of a file](https://man7.org/linux/man-pages/man1/head.1.html)           |
| tail    | :x:                                                                                            | [Displays the end of a file](https://man7.org/linux/man-pages/man1/tail.1.html)                 |
| sort    | :x:                                                                                            | [Sorts lines of text files](https://man7.org/linux/man-pages/man1/sort.1.html)                  |
| uniq    | :x:                                                                                            | [Removes duplicate lines from a sorted file](https://man7.org/linux/man-pages/man1/uniq.1.html) |
| wc      | [![rwc](https://github.com/dgj7/rs_coreutils/actions/workflows/rwc.yml/badge.svg)](/rwc)       | [Prints word, line, and byte counts for files](https://man7.org/linux/man-pages/man1/wc.1.html) |
| tr      | :x:                                                                                            | [Translates or deletes characters](https://man7.org/linux/man-pages/man1/tr.1.html)             |
| grep    | :x:                                                                                            | [Searches for patterns in files](https://man7.org/linux/man-pages/man1/grep.1.html)             |
| echo    | :x:                                                                                            | [Displays text on the terminal](https://man7.org/linux/man-pages/man1/echo.1.html)              |
| printf  | :x:                                                                                            | [Formats and prints data](https://man7.org/linux/man-pages/man1/printf.1.html)                  |

## File & Directory Management
| Utility | Status                                                                                            | Manual                                                                            |
|---------|---------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------|
| ls      | :x:                                                                                               | [Lists directory contents](https://man7.org/linux/man-pages/man1/ls.1.html)       |  
| cat     | :x:                                                                                               | [Concatenates and prints files](https://man7.org/linux/man-pages/man1/cat.1.html) |                         
| cp      | :x:                                                                                               | [Copies files](https://man7.org/linux/man-pages/man1/cp.1.html)                   |                              
| mv      | :x:                                                                                               | [Moves or renames files](https://man7.org/linux/man-pages/man1/mv.1.html)         |                              
| rm      | :x:                                                                                               | [Removes files or directories](https://man7.org/linux/man-pages/man1/rm.1.html)   |                              
| mkdir   | :x:                                                                                               | [Creates directories](https://man7.org/linux/man-pages/man1/mkdir.1.html)         |                              
| rmdir   | :x:                                                                                               | [Removes empty directories](https://man7.org/linux/man-pages/man1/rmdir.1.html)   |                              
| ln      | :x:                                                                                               | [Creates links to files](https://man7.org/linux/man-pages/man1/ln.1.html)         |                              
| chown   | :x:                                                                                               | [Changes file ownership](https://man7.org/linux/man-pages/man1/chown.1.html)      |                              
| chmod   | :x:                                                                                               | [Changes file permissions](https://man7.org/linux/man-pages/man1/chmod.1.html)    |                              
| touch   | [![rtouch](https://github.com/dgj7/rs_coreutils/actions/workflows/rtouch.yml/badge.svg)](/rtouch) | [Updates file timestamps](https://man7.org/linux/man-pages/man1/touch.1.html)     |          

## Disk & System Information
| Utility  | Status | Manual                                                                                       |
|----------|--------|----------------------------------------------------------------------------------------------|
| df       | :x:    | [Shows free disk space on filesystems](https://man7.org/linux/man-pages/man1/df.1.html)      |
| du       | :x:    | [Estimates file space usage](https://man7.org/linux/man-pages/man1/du.1.html)                |
| uname    | :x:    | [Prints system information](https://man7.org/linux/man-pages/man1/uname.1.html)              |
| hostname | :x:    | [Shows or sets the system's hostname](https://man7.org/linux/man-pages/man1/hostname.1.html) |

## Process & User Management

| Utility | Status | Manual                                                                                        |
|---------|--------|-----------------------------------------------------------------------------------------------|
| ps      | :x:    | [Lists running processes](https://man7.org/linux/man-pages/man1/ps.1.html)                    |
| kill    | :x:    | [Sends a signal to a process](https://man7.org/linux/man-pages/man1/kill.1.html)              |
| su      | :x:    | [Runs a command as another user](https://man7.org/linux/man-pages/man1/su.1.html)             |
| id      | :x:    | [Prints user and group IDs](https://man7.org/linux/man-pages/man1/id.1.html)                  |
| nice    | :x:    | [Modifies a command's scheduling priority](https://man7.org/linux/man-pages/man1/nice.1.html) |

## File Manipulation & Conversion
| Utility   | Status | Manual                                                                                        |
|-----------|--------|-----------------------------------------------------------------------------------------------|
| dd        | :x:    | [Copies and converts a file](https://man7.org/linux/man-pages/man1/dd.1.html)                 |
| md5sum    | :x:    | [Computes and checks message digests](https://man7.org/linux/man-pages/man1/md5sum.1.html)    |
| sha256sum | :x:    | [Computes and checks message digests](https://man7.org/linux/man-pages/man1/sha256sum.1.html) |
| split     | :x:    | [Splits a file into smaller pieces](https://man7.org/linux/man-pages/man1/split.1.html)       |
| base64    | :x:    | [base-64 encode input](https://man7.org/linux/man-pages/man1/base64.1.html)                   |

## Other Applications (Because I Said So)
| Utility | Status                                                                                      | Manual                                                              |
|---------|---------------------------------------------------------------------------------------------|---------------------------------------------------------------------|
| cal     | [![rcal](https://github.com/dgj7/rs_coreutils/actions/workflows/rcal.yml/badge.svg)](/rcal) | [print calendars](https://man7.org/linux/man-pages/man1/cal.1.html) |

---
