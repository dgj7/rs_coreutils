# Coreutils - Rust
Rewriting coreutils with Rust.

Don't use any of these programs in lieu of the built-in commands; this project is for learning purposes only, and _is not a suitable replacement_.

## Directory
* ![rcal](https://github.com/dgj7/rs_coreutils/actions/workflows/rcal.yml/badge.svg) (_[code](/rcal)_) - rewrite of [cal](https://man7.org/linux/man-pages/man1/cal.1.html)
* ![rtouch](https://github.com/dgj7/rs_coreutils/actions/workflows/rtouch.yml/badge.svg) (_[code](/rtouch)_) - rewrite of [touch](https://man7.org/linux/man-pages/man1/touch.1.html)
* ![rwc](https://github.com/dgj7/rs_coreutils/actions/workflows/rwc.yml/badge.svg) (_[code](/rwc)_) - rewrite of [wc](https://man7.org/linux/man-pages/man1/wc.1.html) ([sauce](https://github.com/coreutils/coreutils/blob/master/src/wc.c))

## Todo
* create a to-do list for each project; it should contain a todo for
  * each command line arg/feature; each should have an associated end-to-end test
* externalize shared types in a `common` project of some kind

---
