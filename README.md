<h2 align="center"> ━━━━━━  ❖  ━━━━━━ </h2>

<!-- BADGES -->
<div align="center">
   <p></p>
   
   <img src="https://img.shields.io/github/stars/dotzenith/TheSeptaTimes.rs?color=F8BD96&labelColor=302D41&style=for-the-badge">   

   <img src="https://img.shields.io/github/forks/dotzenith/TheSeptaTimes.rs?color=DDB6F2&labelColor=302D41&style=for-the-badge">   

   <img src="https://img.shields.io/github/repo-size/dotzenith/TheSeptaTimes.rs?color=ABE9B3&labelColor=302D41&style=for-the-badge">
   
   <img src="https://img.shields.io/github/commit-activity/y/dotzenith/TheSeptaTimes.rs?color=96CDFB&labelColor=302D41&style=for-the-badge&label=COMMITS"/>
   <br>
</div>

<p/>

---

### ❖ TheSeptaTimes.rs

TheSeptaTimes.rs is an easy-to-use commandline utility to fetch information about regional SEPTA trains

  <img src="https://github.com/dotzenith/dotzenith/blob/main/assets/TheSeptaTimes/septa.gif" alt="septa gif">

---

### ❖ Installation

#### Brew
```sh
brew tap dotzenith/tap
brew install TheSeptaTimes
```

#### Binaries
Pre-Compiled binaries for Linux and MacOS are available in [Releases](https://github.com/dotzenith/TheSeptaTimes.rs/releases)

#### Cargo
```sh
cargo install the-septa-times
```

#### Source
- First, install [rust](https://rustup.rs/)
```sh
git clone https://github.com/dotzenith/TheSeptaTimes.rs.git
cd TheSeptaTimes.rs
cargo build --release
./target/release/tst
```

---

### ❖ Usage

```sh
Usage: tst [OPTIONS] COMMAND [ARGS]...

Options:
  --help  Show this message and exit.

Commands:
  next      Search for the next train going from an origin to a destination
  arrivals  Find the next arrivals at a given train station
  train     Track a given train using it's number
  stations  Get all valid station names
  refresh   Refresh the cache for station names
```

> Get times for the next two trains that go from a given train station to another
```sh
tst next '30th Street Station' 'North Philadelphia'
```

> List the next 6 arrivals at a given train station
```sh
tst arrivals '30th Street Station' 6
```

> Take a look at any given train's schedule using the train number
```sh
tst train 9374
```

> Get all valid train station names
```sh
tst stations
```

> Refresh the cache for station names
```sh
tst refresh
```

### ❖ Advanced Usage

Since `tst` requires the "correct" station names, it can often be a little cumbersome. This can be alleviated with the power of pipes and [rg](https://github.com/BurntSushi/ripgrep) (or grep)

```bash
#!/usr/bin/env bash
tst next "$(tst stations | rg -i $1 | head -1)" "$(tst stations | rg -i $2 | head -1)" $3
```

Now you can call the script like so:
```
next "30th" "villa" 5       # The same as: tst next "30th Street Station" "Villanova" 5
```

The script above can be easily adapted for `tst arrivals` as well

---

### ❖ What's New? 
0.1.2 - Initial release

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
