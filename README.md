# memento-mori-rs

Memento mori means "remember that you must die" in the language of Latin. 

It serves to remind us of our own mortality, of the inevitable transformation of life into death.

## Usage
```
Usage: memento-mori-cli [OPTIONS] --birthday <BIRTHDAY>

Options:
  -b, --birthday <BIRTHDAY>
  -d, --death-age <DEATH_AGE>  [default: 90]
  -t, --time-unit <TIME_UNIT>  [default: month] [possible values: week, month]
  -h, --help                   Print help
  -V, --version                Print version
```

## Build
```
$ cargo build --release
```

## Run
```
$ ./target/release/memento-mori-cli -b <birthday in yyyy-MM-dd>
```

See all the options in the help:
```
$ ./target/release/memento-mori-cli --help
```

