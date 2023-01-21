# Rust for LabVIEW Programmers

## Overview

As a LabVIEW Architect and someone who has made a career out of writing LabVIEW code for Windows, NI RT Linux and NI FPGA for over 10 years, I recently fell in love with the programming language Rust.  I have been branching out into new languages since I picked up my CLA three years ago - first JavaScript, then TypeScript, Python and lately Rust.  

Rust certainly has an intimidating reputation for its steep learning curve, but of all these languages I've found the greatest affinity for Rust.  I believe this is partly because it shares some crucial DNA with things I particularly enjoy about LabVIEW, and excels at some of the things I find most hard work with LabVIEW.

---

## Things in Common

### Strict type system & Compile time checks

Rust's type system, particularly its numeric options, are pleasantly familiar.  I'm very used to thinking about the size and sign of numerics in LabVIEW and this always feels like a sacrifice in other languages like JavaScript only having a 'number' type and Python's 'float' or 'int' types.

```rust
let unsigned: u32 = 500;
let signed: i8 = -4;
let float: f32 = 3.1415;
```

In both languages for a little bit of effort defining types you get a lot of benefits to your developer experience in the form of compile time static-analysis and self-documenting code blocks.

![labVIEW Random Number](./img/labVIEW-random-number.png)

![Rust Random Number](./img/rust-random-number.png)

### Compile time checks

I'm used to a broken run arrow telling me my types aren't compatible and find it frustrating that other languages will let you get away with things that it should know aren't going to work.  Rust goes even further than LabVIEW with its static analysis and assurance against things like race conditions.

### Enums

Rust's handling of enums will also be familiar to a labVIEW Programmer.  The state machine pattern is all over LabVIEW code, and enforcing that each case is handled explicitly or via a default is a big developer boon that Rust is equally pedantic about.

![LabVIEW Match filling](./img/labVIEW-match.gif)

![Rust Match filling](./img/rust-match.gif)

Rust goes a step further with enums though, by allowing you to add additional methods and data to enum states, which opens up a whole interesting world of possible design patterns.

```rust
enum State {
    Init,
    Error(Result<(), u32>),
    Read((u8, u32)),
    Write(Vec<u8>),
    Close,
}

fn match_test(state: State) {
    match state {
        State::Init => todo!(),
        State::Error(error) => todo!(),
        State::Read((channel, samples)) => todo!(),
        State::Write(bytes) => todo!(),
        State::Close => todo!(),
    }
}
```

### High-level assistance, but go as low as you need

The LabVIEW standard library is pretty packed with functions that help you solve common coding challenges, and whilst Rust's standard library contains less (by design) it is easily extended by libraries, called [crates](https://www.crates.io).

### Error handling framework

### Data ownership

### High performance

---

## Wins for Rust

### Open-source

Of course, its hard to argue with the free pricetag on an open-source language, and one that is gaining more and more traction across [major tech companies](https://www.rust-lang.org/sponsors), even becoming the [second official language of Linux](https://www.techzine.eu/news/devops/69453/rust-emerges-as-linuxs-second-official-language/) recently.  The community is welcoming, active and growing and of course Rust keeps getting voted most loved language on Stack Overflow.  [Stable](https://blog.rust-lang.org/2014/10/30/Stability.html) feature releases are on a regular 6 weekly cycle, preceeded by unstable (Nightly) releases for testing new features.

### Run anywhere

Rust code compiles to a binary, as does LabVIEW, however it has no external runtime dependency and can be compiled to run on Windows, Linux, Mac, Mobile, and embedded chips such as the STM32.  Frameworks like [Tauri](https://tauri.app/) create standalone cross-platform GUI applications.

### Library ecosystem

One of my major bugbears with LabVIEW these days is the difficulty creating isolated environments to develop code.  When it was first designed, globally installing libraries was the done thing, but this makes library version management painful, especially if working across multiple versions of LabVIEW.  Rust libraries are all installed and managed on a project by project basis, via a cargo.toml file that specifies the external libraries used and their versions.

### IDEs and source-control

Text based programming comes with knock-on benefits by playing nicely with all major IDEs and source-control systems, though this isn't unique to Rust.

---

## Wins for LabVIEW

### Lower skill floor

The learning curve is there with Rust. Though it also exist for LabVIEW, it is definitely easier to get started building and running an application, which is what has made LabVIEW so accessible for Engineers and Scientists without a Computer Science background.  The skill requirement gap is largest when dealing with User Interfaces, with LabVIEW's drag and drop graphical interface.  However, with its non-vector graphics, there definitely comes a crossover where you begin fighting with LabVIEW to create more complex, dynamic interfaces that users might expect in 2023.

### Parallelism

LabVIEW has been described as a "massively parallel" language, it is trivial to 

### Hardware ecosystem

The biggest thing in LabVIEW's favour is its catalogue of APIs to many common hardware types, this being the bread and butter of Test & Measurement activities, as well as native integration with NI DAQ, RIO, PXI, FPGA, etc.
