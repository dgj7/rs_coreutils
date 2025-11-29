# Coreutils - Rust
Rewriting coreutils with Rust.

Don't use any of these programs in lieu of the built-in commands; this project is for learning purposes only, and _is not a suitable replacement_.

## Text Processing And Input
| Utility | Status                                                                                   | Manual                                                                                          |
|---------|------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------|
| head    | :x:                                                                                      | [Displays the beginning of a file]()                                                            |
| tail    | :x:                                                                                      | [Displays the end of a file]()                                                                  |
| sort    | :x:                                                                                      | [Sorts lines of text files]()                                                                   |
| uniq    | :x:                                                                                      | [Removes duplicate lines from a sorted file]()                                                  |
| wc      | [![rwc](https://github.com/dgj7/rs_coreutils/actions/workflows/rwc.yml/badge.svg)](/rwc) | [Prints word, line, and byte counts for files](https://man7.org/linux/man-pages/man1/wc.1.html) |
| tr      | :x:                                                                                      | [Translates or deletes characters]()                                                            |
| grep    | :x:                                                                                      | [Searches for patterns in files]()                                                              |
| echo    | :x:                                                                                      | [Displays text on the terminal]()                                                               |
| printf  | :x:                                                                                      | [Formats and prints data]()                                                                     |

## File & Directory Management
| Utility | Status                                                                                            | Manual                                                                        |
|---------|---------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------|
| ls      | :x:                                                                                               | [Lists directory contents]()                                                  |  
| cat     | :x:                                                                                               | [Concatenates and prints files]()                                             |                         
| cp      | :x:                                                                                               | [Copies files]()                                                              |                              
| mv      | :x:                                                                                               | [Moves or renames files]()                                                    |                              
| rm      | :x:                                                                                               | [Removes files or directories]()                                              |                              
| mkdir   | :x:                                                                                               | [Creates directories]()                                                       |                              
| rmdir   | :x:                                                                                               | [Removes empty directories]()                                                 |                              
| ln      | :x:                                                                                               | [Creates links to files]()                                                    |                              
| chown   | :x:                                                                                               | [Changes file ownership]()                                                    |                              
| chmod   | :x:                                                                                               | [Changes file permissions]()                                                  |                              
| touch   | [![rtouch](https://github.com/dgj7/rs_coreutils/actions/workflows/rtouch.yml/badge.svg)](/rtouch) | [Updates file timestamps](https://man7.org/linux/man-pages/man1/touch.1.html) |          

## Disk & System Information
| Utility  | Status | Manual                                   |
|----------|--------|------------------------------------------|
| df       | :x:    | [Shows free disk space on filesystems]() |
| du       | :x:    | [Estimates file space usage]()           |
| uname    | :x:    | [Prints system information]()            |
| hostname | :x:    | [Shows or sets the system's hostname]()  |

## Process & User Management

| Utility | Status | Manual                                       |
|---------|--------|----------------------------------------------|
| ps      | :x:    | [Lists running processes]()                  |
| kill    | :x:    | [Sends a signal to a process]()              |
| su      | :x:    | [Runs a command as another user]()           |
| id      | :x:    | [Prints user and group IDs]()                |
| nice    | :x:    | [Modifies a command's scheduling priority]() |

## File Manipulation & Conversion
| Utility   | Status | Manual                                  |
|-----------|--------|-----------------------------------------|
| dd        | :x:    | [Copies and converts a file]()          |
| md5sum    | :x:    | [Computes and checks message digests]() |
| sha256sum | :x:    | [Computes and checks message digests]() |
| split     | :x:    | [Splits a file into smaller pieces]()   |
| base64    | :x:    | [base-64 encode input]()                |

## Other Applications (Because I Said So)
| Utility | Status                                                                                      | Manual                                                              |
|---------|---------------------------------------------------------------------------------------------|---------------------------------------------------------------------|
| cal     | [![rcal](https://github.com/dgj7/rs_coreutils/actions/workflows/rcal.yml/badge.svg)](/rcal) | [print calendars](https://man7.org/linux/man-pages/man1/cal.1.html) |

---
