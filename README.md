# Ferrous Systems' Rust Training

This is free workshop material produced by Ferrous Systems for trainings. See our [ferrous-systems.com/training](https://ferrous-systems.com/training) for more details or a custom quote.

The material is created for people with anywhere from zero Rust experience (but with a programming background) to advanced Rust knowledge.

Ferrous Systems offers a large Rust curriculum for both beginner and advanced Rust developers.

Ferrous Systems specialises in custom, topic focused workshops for enterprises starting or expanding their use of Rust. Supplemental to our courses, we offer ongoing help and feedback.

## Overview

The materials are presented as a set of small, self-contained lessons on a specific topic. Note that lessons might be revised, extended, or removed, when necessary to keep material up to date or relevant to new audiences.

We assemble these lessons into various programmes for our commercial trainings. We can also provide custom lessons - please [reach out](https://ferrous-systems.com/contact) if that is of interest.

## Reading the material

This material is organised as an [`mdbook`](https://crates.io/crates/mdbook), which we also render to [`reveal.js`](https://revealjs.com) slides using an open-source tool we wrote called [`mdslides`](https://crates.io/crates/mdslides).

You can:

* Browse [the chapters of the book on Github](./training-slides/src/SUMMARY.md)
* Clone the repo, and build the book (see [Building the material locally](#building-the-material-locally))
* Download the slides in both slide-deck and book format, from the [releases area](https://github.com/ferrous-systems/rust-training/releases)

## Building the material locally

This slide deck is an [`mdbook`](https://crates.io/crates/mdbook) that is also converted into slides using a tool we wrote called [`mdslides`](https://crates.io/crates/mdslides).

To build as a book, run `mdbook` in the usual fashion:

```console
$ cargo install mdbook
$ cargo install mdbook-mermaid
$ cd ./training-slides
$ mdbook build
2023-05-03 13:24:46 [INFO] (mdbook::book): Book building has started
2023-05-03 13:24:46 [INFO] (mdbook::book): Running the html backend
$ ls -l book/index.html
-rw-r--r--  1 jonathan  staff  13573 May  3 11:48 book/index.html
```

You could use `mdbook serve` instead to start a webserver that serves up the book and also rebuilds the book every time you change a file.

To verify that every code example in the book compiles as expected, run:

```sh
mdbook test
```

To convert the book to slides, run `mdslides` like this:

```console
$ cargo install mdslides
$ cd ./training-slides
$ mdslides --template ./template.html --output-dir ./slides --mdbook-path . --index-template ./index-template.html 
$ ls -l slides/index.html 
-rw-r--r--@ 1 jonathan  staff  46172 May  3 11:48 slides/index.html
```

For brevity, all the above actions are packed into a shell script, which is what the CI workflow executes:

```sh
cd ./training-slides
./build.sh
```

The `mdslides` tool doesn't have a `serve` mode, so you'll need to either open the HTML files from disk, or run your own webserver. You will need a working internet connection as the HTML files load revealjs and other dependencies from a Content Delivery Network.

```sh
# If you have Python 3 installed
python3 -m http.server -d ./slides
# If you don't have Python 3 installed, try:
cargo install https
httplz ./slides
# Both serve on http://localhost:8000
```

## Credits

The development of this course is financed by Ferrous Systems. They are open sourced as a contribution to the growth of the Rust language.

If you wish to fund further development of the course, [book a training](https://ferrous-systems.com/training)!

## License

[![Creative Commons License](https://i.creativecommons.org/l/by-sa/4.0/88x31.png)](http://creativecommons.org/licenses/by-sa/4.0/)

This work is licensed under a [Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/).

We encourage the use of this material, under the terms of the above license, in the production and/or delivery of commercial or open-source Rust training programmes.

Copyright (c) Ferrous Systems, 2023
