User Guide
---------

This guide will give you an overview of what you can do with vocabulist.
This guide assumes that __vocabulist__ is [installed](README.md#installation).

To get started with __vocabulist__ you'll want to `import` a few text files.

```
$ vocabulist_rs import [PATH] # PATH is the path to a .txt file or a directory of .txt files.
```

Next, to verify that the files have been imported, you can use the `list` command.

```
$ vocabulist_rs list [NUMBER] # NUMBER is the number of terms to list.
```

There are several different ways to change the results returned by the `list` command.
Check `list --help` for more details.

Before generating the flashcards you can `exclude` terms.

```
$ vocabulist_rs exclude [PATH] # PATH is the path to a .txt file of terms separated by newlines
```

If you mess up, you can use use the `include` command to revert the changes.

```
$ vocabulist_rs include [PATH] # PATH is the path to a .txt file of terms separated by newlines
```

Finally, you can generate the flashcards. 
This command requires the anki add-on [AnkiConnect](https://github.com/FooSoft/anki-connect).

```
$ vocabulist_rs generate [NUMBER] # NUMBER is the number of flashcards to generate
```

If want to add flashcards to an existing anki deck make sure you run the `sync` command first.

```
$ vocabulist_rs sync
```

### Configuration File

It's possible you might want to change some of the default settings.
You can do this using the `config.toml` file in the `.vocabulist_rs` directory in `$HOME`.

Here is an example `config.toml`:

```
$ cat config.toml
database_path = "/Users/example/.vocabulist_rs/vocabulist_rs.db"   # the path to the database (this will be created automatically)
dictionary_path = "/Users/example/.vocabulist_rs/jmdict.db"        # the path to the jmdict.db

[anki]
deck_name = "Vocabulist V2" # the name of the deck to generate flashcards in
model_name = "Vocabulist"   # the name of the anki model the cards will use
allow_duplicates = false    # whether duplicate terms are allowed
duplicate_scope = "deck"    # whether to search for duplicates in the deck or the whole library
audio = true                # whether to have AnkiConnect pull down audio for the flashcards
tags = ["vocabulist"]       # the tags for the cards

# The fields in the anki model mode_name.
# Consists of two vectors of equal length.
# The first vector contains the names of the fields.
# The second vector contains the values that corresponds to the fields.
# the allowed values are:
# expression - the term for the card (may contain kanji)
# reading - the reading in hiragana or katakana for the expression
# definition - the definition for the expression
# sentence - one sentence for the expression
# audio - the field that will play the audio when shown
fields = [["Expression","Reading", "Definition", "Sentence", "Audio"], ["expression", "reading", "definition", "sentence", "audio"]]o
```

Please note.
Right now the anki model must contain a field to hold the `expression` value, otherwise some of the features such as `sync` may not work.
