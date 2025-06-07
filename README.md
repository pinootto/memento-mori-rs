# memento-mori-rs

Memento mori means "remember that you must die" in the language of Latin. 

It serves to remind us of our own mortality, of the inevitable transformation of life into death.

This project has two executable files: memento-mori-cli and memento-mori-web.

## Build
```
$ cargo build --release
```

## CLI

### Usage
```
Usage: memento-mori-cli [OPTIONS] --birthday <BIRTHDAY>

Options:
  -b, --birthday <BIRTHDAY>
  -d, --death-age <DEATH_AGE>  [default: 90]
  -t, --time-unit <TIME_UNIT>  [default: month] [possible values: week, month]
  -h, --help                   Print help
  -V, --version                Print version
```

### Run
```
$ ./target/release/memento-mori-cli -b <birthday in yyyy-MM-dd>
```

See all the options in the help:
```
$ ./target/release/memento-mori-cli --help
```

## Web

### Usage
```
Usage: memento-mori-web [OPTIONS]

Options:
  -p, --port <PORT>  [default: 4001]
  -h, --help         Print help
  -V, --version      Print version
```

### Run
```
RUST_LOG=info ./target/release/memento-mori-web
```
test in localhost:
```
curl "http://localhost:4001/calendar?birthday=1985-07-05"
curl "http://localhost:4001/calendar?birthday=1985-07-05&death_age=80&time_unit=Week"
curl "http://localhost:4001/calendar?birthday=1985-07-05&death_age=80&time_unit=Month"
```

### Demo
```
[Link to demo](https://gioyingtec.com/memento-mori/calendar?birthday=1975-07-02&death_age=70&time_unit=Week)
```
