# Inliner

`inliner` takes a file and outputs a Rust `const` variable declaration containing all the bytes in that
file. This declaration can then be pasted into your Rust code.

```
Convert a file into a `const` Rust array for easier inclusion into source files.

"How about `include_bytes!()`?

This macro requires you to have non source files in your source tree, which is not always desirable (the build
can break if you move these files around, you might want to make these binary files not searchable, etc...).

Usage: inliner.exe [OPTIONS] <FILE_NAME>

Arguments:
  <FILE_NAME>

Options:
  -c, --columns <COLUMNS>              Max number of columns [default: 80]
  -i, --indent <INDENT>                Number of spaces for indentation [default: 4]
  -v, --variable <VARIABLE>  Name of the variable [default: derived from the file name]
  -h, --help                           Print help
  -V, --version                        Print version
```

For example:

```
$ inliner --columns 60 --indent 2 --variable rom dsk.rom 

const rom: [u8; 237] = [
  0x23, 0x47, 0x68, 0x69, 0x64, 0x72, 0x61, 0x20, 0x4C,
  0x6F, 0x63, 0x6B, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D,
  ...
];
```