<h1 align="center"> ━━━━━━  ❖  ━━━━━━ </h1>

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

## ❖ TheSeptaTimes.rs

TheSeptaTimes.rs is an easy-to-use commandline utility to fetch information about regional SEPTA trains

  <img src="https://github.com/dotzenith/dotzenith/blob/main/assets/TheSeptaTimes/septa.gif" alt="septa gif">

---

## ❖ Installation

#### Shell
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/dotzenith/TheSeptaTimes.rs/releases/latest/download/the-septa-times-installer.sh | sh
```

#### Brew
```sh
brew tap dotzenith/tap
brew install the-septa-times
```

#### Powershell
```sh
irm https://github.com/dotzenith/TheSeptaTimes.rs/releases/latest/download/the-septa-times-installer.ps1 | iex
```

#### Cargo
```sh
cargo install the-septa-times
```

#### Binaries
Pre-Compiled binaries for linux, mac, and windows are available in [Releases](https://github.com/dotzenith/TheSeptaTimes.rs/releases)

#### Source
- First, install [rust](https://rustup.rs/)
```sh
git clone https://github.com/dotzenith/TheSeptaTimes.rs.git
cd TheSeptaTimes.rs
cargo build --release
./target/release/tst
```

---

## ❖ Usage

```
A CLI application for the SEPTA API

Usage: tst <COMMAND>

Commands:
  next      Search for the next train going from an origin to a destination
  arrivals  Find the next arrivals at a given train station
  train     Track a given train
  stations  Get all valid station names
  extra     All of the extra endpoints added by SepatPlusPlus
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### ❖ Base commands

#### Get times for the next two trains that go from a given train station to another:
```sh
tst next '30th Street Station' 'North Philadelphia'
```

#### Since `tst` uses fuzzy matching, you can also do:
```sh
tst next 'suburban' '30th'
```
> Setting `SeptaPlusPlusURL` as seen [below](#-extra-commands-provided-by-septaplusplus) will help make the fuzzy matching more accurate, but it is optional

#### List the next 6 arrivals at a given train station:
```sh
tst arrivals '30th Street Station' --count 6
```

#### Take a look at any given train's schedule using the train number:
```sh
tst train 9374
```

#### Get all valid train station names:
```sh
tst stations
```

### ❖ Extra commands provided by [SeptaPlusPlus](https://github.com/dotzenith/SeptaPlusPlus)

These commands require endpoints provided by [SeptaPlusPlus](https://github.com/dotzenith/SeptaPlusPlus).
`tst` requires the `SeptaPlusPlusURL` environment variable to be set like:

```sh
export SeptaPlusPlusURL="https://septa.jawn.website/api"
```

#### Get all lines supported by the `tst extra schedule` command:
```sh
tst extra lines
```

#### Get all stations on a given track, as supported by the `tst extra schedule` command:
```sh
tst extra stations TRE          # On the Trenton line
```

#### Get train schedule going from one station to another on a given line
> This command also uses fuzzy matching so station names do not need to be exact
```sh
tst extra schedule TRE "Trenton" "Gray 30th Street" inbound weekday
```

---

## ❖ What's New? 
0.8.0 - Use [SeptaPlusPlus](https://github.com/dotzenith/SeptaPlusPlus) for station caching and remove manual cache refresh

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
