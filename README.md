# ppgen

A cheap and cheerful passphrase generator (for those who don't have a pile of D6s on hand).

```
Usage: ppgen [OPTIONS] --wordlist <WORDLIST>

Options:
  -w, --wordlist <WORDLIST>  Path to EFF wordlist
  -s, --size <SIZE>          Number of words in passphrase [default: 4]
  -h, --help                 Print help information
  -V, --version              Print version information
```

This requires a local copy of the [EFF Long Wordlist (.txt)](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt) in order to work.

This program automates the EFF's recommended [dice-based generator](https://www.eff.org/dice).
