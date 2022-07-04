# ats-sport results extractor

Extract results from ats-sport.com, print them to stdout in CSV format.

Note that there are some garbage printed to stderr...

Usage:

```
cargo run -- 'https://www.ats-sport.com/resultats_epreuve.php?epreuve=7638&parcours=20076' > results.csv
```
