Bucket List CLI
========================================
Bucket List CLI is a CLI tool which maintains a bucket list weighting items automatically.

<!-- TOC --->
- [Why use Bucket List CLI?](#why-use-bucket-list-cli)
- [Installation](#installation)
- [How to Use](#how-to-use)


# Why use Bucket List CLI?
In recent years, many people are too busy with a huge number of todo.
Due to this, we cannot spare time for what we want to do (called bucket list), even though these are very important for our life and career in the long run.
On the other hand, there are also too many items in the bucket list and we don't have enough time to do all.
Then, we need to prioritize them and focus more important items.

Bucket List CLI maintains a bucket list, but it isn't just a bucket list.
It weights our items automatically based on the simple but powerful principles:
- The more we encounter it, the more important it is.
- As items gets older, it becomes less important.

Prioritization is a heavier decision making than you think.
Bucket List CLI automates prioritization and makes us to focus more important items and forget less important ones.
Let's enrich your life with Bucket LIST CLI!!


# Installation
Bucket List CLI is written in Rust, so you can build and install with `cargo`.
To build, you need to install Rust as a prerequisite.
```
$ git clone https://github.com/zulinx86/bucketlist_cli
$ cd bucketlist_cli
$ cargo install --locked --path .
$ ./target/release/bl --help
```
You can move the binary file named "bl" to anywhere under your `$PATH`.


# How to Use
Bucket List CLI saves its data into `~/.bucketlist/data.json` locally.

## Add a new item
Use `bl add` command not only when you want to add a new item but also when you encounter the item already in the list.
Using this command to the same item for more times makes its priority higher.
```
$ bl add <item name>
```

## Delete an item
When you have done it or there's no need to do it, use `bl del` command.
```
$ bl del <item name>
```

## List items (sorted by priority)
As `bl ls` command shows items sorted by priority, you can use this to decide the next item to do.
```
$ bl ls
```

After time has passed since items were added (e.g. 30 days after the item was added first time), the items become invisible by default.
If you want to display them, use `--all` option.
```
$ bl ls --all
```

## Annotate an item
You can add a note to the existing items by this command.
```
$ bl note <item name> <note>
```

Note that the content of the note is overwritten every time.
If you want to preserve the previous one, you need to put them together.


# License
Licensed under [MIT license](https://github.com/zulinx86/bucketlist_cli/blob/main/LICENSE).
