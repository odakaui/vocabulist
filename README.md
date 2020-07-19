vocabulist
------------

![Rust](https://github.com/odakaui/vocabulist/workflows/Rust/badge.svg)

So, you're learning Japanese.

You've mastered Hiragana and Katakana and you've decided to build your vocabulary.
You do some research online and everyone says, "Use flashcards!"
So, you do a bit of searching and download Anki.

Now, you're all set.

Armed with your knowledge of Hiragana and your trusty Anki install, you set out to create flashcards.

You go online and find yourself a Japanese frequency list. 
Then, starting from the top, you pick an term, look up the definition on jisho.org, and finally input the data into Anki.

But you're lazy, and after about 100 expressions, you decide that learning Japanese actually isn't for you and decide to pick up Spanish instead.

Or maybe, you're more displined than the rest of us and input 1000s of terms into Anki.
You eagerly start memorizing the terms, only to find that you struggle to learn them with no context.

Or even better, you memorize the flashcards 1000s of flashcards. 
However, when you go to start reading you find that the terms you've memorized don't show up in the text you're reading.

Well, here's one possible solution to your problems in an sea of infinite solutions.

Introducing __vocabulist__.
A Japanese frequency list personalized just for you.

1. Simply, copy the text you want to read into text file(s). 
2. Import the files into __vocabulist__.
3. Turn them into flashcards with a single command.

It's as easy as one two three.

__vocabulist__ automatically creates flashcards based on the frequency of the terms in the imported text. 
So you don't have to worry about spending valuable time learning a term that you'll never see again.

### CHANGELOG

Please see the [CHANGELOG](CHANGELOG.md) for a release history.

### Documentation quick links

* [Installation](#installation)
* [User Guide](GUIDE.md)
* [Configuration files](GUIDE.md#configuration-file)

### Installation

Currently the only way to install __vocabulist__ is to clone the repository and build it from scratch.
Make sure you have a [Rust installation](https://www.rust-lang.org/) in order to compile it.
__vocabulist__ compiles with Rust 1.44.1 (stable) or newer.
It tracks the latest stable release of the Rust compiler.

To build __vocabulist__:

```
$ git clone https://github.com/odakaui/vocabulist.git
$ cd vocabulist
$ cargo build --release
$ mv jmdict.db "${HOME}/.vocabulist_rs/"
```

### Features

* Import terms from a .txt file containing Japanese text or a directory of .txt files.
* List [x] terms in the database.
* Generate [x] flashcards starting from the most frequent.
* Sync the database with Anki to avoid creating flashcards for duplicate terms.
* Exclude/Include terms in list and flashcard generation functionality.

__vocabulist__ is a work in progress. 
Right now it has a few short comings. 
It only works with Japanese.
It only works with the JMdict sqlite3 database provided in the repository.
It only works with anki.

I have plans to address some of these issues in the future.
