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
  arrivals  Find the next arrivals at a given train station
  next      Search for the next train going from an origin to a destination
  stations  Get all valid station names
  train     Track a given train using it's number
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

---

### ❖ What's New? 
0.1.2 - Initial release

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
