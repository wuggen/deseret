# Deseret

A super-simple command line tool to translate Deseret script to IPA.

## Usage

Without arguments, `deseret` will read UTF-8 text from standard input and write
transliterated text to standard output. It will read the _entire_ standard input, until
end-of-stream, before producing any output; that is, you can't really use it
interactively.

Alternatively, you can specify an input and/or an output file:

```
deseret --in deseret_input.txt --out ipa_output.txt

# Equivalently:
deseret -i deseret_input.txt -o ipa_output.txt
```

If `--in` is omitted, it will read from stdin as usual; similarly, if `--out` is omitted,
it will write to stdout as usual.

Finally, you can also pass Deseret text as an argument:

```
deseret "๐๐ถ๐ฎ๐๐ฟ๐, ๐ป๐ถ๐ฎ๐๐ฟ๐, ๐๐ฎ๐ป๐ ๐๐ป๐ช๐"

# Outputs: twษชลkl, twษชลkl, lษชtl stษหr
```

This is mutually exclusive with the `--in` option.

`deseret --help` or `deseret -h` will display a brief description of these options.
