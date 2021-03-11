# spades-runner
![spades-runner](https://github.com/hhandika/spades-runner/workflows/Tests/badge.svg)
[![Build Status](https://www.travis-ci.com/hhandika/spades-runner.svg?branch=main)](https://www.travis-ci.com/hhandika/spades-runner)

`spades-runner` is a command-line application to batch assembly next-gen sequencing reads using SPAdes. It aims to simplify batch assembly process with minimal to no-configuration file. 

# Table of Contents
- [Quick Start](#quick-start)
- [Installation](#installation)


# Quick Start

If your folder structure similar to [fastp-runner](https://github.com/hhandika/fastp-runner) or [phyluce](https://phyluce.readthedocs.io/en/latest/) or use either program to clean your sequencing reads, you can use the auto detection command. The app will detect your folder structure. Therefore, there is no need to use a configuration file. 

```
spr auto -d [you-clean-read-folder]
```

An option to use a configuration file is also available. You can use a two-column csv:

|Samples        | Path                                      |
|---------------|-------------------------------------------|
|some_species   |clean_reads/some_species/trimmed_reads/    |
|another_species|clean_reads/another_species/trimmed_reads/ |

Or using ini format:

```
[samples]
some_species:clean_reads/some_species/trimmed_reads/
another_species:clean_reads/another_species/trimmed_reads/
```

Then, save your configuration file. The extension for your file does not matter, you could just save it as txt. The command to run spade-runner using a configuration file is as below:

```
spr assembly -i [path-to-your-config-file]
```

For example

```
spr assembly -i bunomys_assembly.config
```

You can check if the app correctly detect your reads using the `dry run` option:

```
spr auto -d [your-clean-read-folder] --dry
```

or 

```
spr assembly -i [path-to-your-config-file] --dry
```

For more options:

```
spr --help
```

## Installation
Installation is similar to [simple-qc](https://github.com/hhandika/simple-qc). For now, please follow the instruction in simple-qc. Then, check if SPAdes is already installed in your system:

```
spr check
```

It will shows your system information and SPAdes version you have. This is an output example from my computer:

```
System Information
Operating system        : openSUSE Tumbleweed 20210210
Kernel version          : 5.10.12-1-default
Available cores         : 12
Available threads       : 24
Total RAM               : 31 Gb

Dependencies:
[OK]    SPAdes genome assembler v3.15.1
```

If it shows `[NOT FOUND`], then install SPAdes. You can find the instruction to install SPAdes [here](https://cab.spbu.ru/software/spades/). After installation, you should `check` again if the app can recognize SPAdes installation.

# Commands

All available sub-command options:

```
USAGE:
    spr <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    assembly    Runs SPAdes
    auto        Auto find clean reads and assembly them
    check       Checks if SPAdes is installed
    clean       Cleans unused SPAdes files.
    help        Prints this message or the help of the given subcommand(s)
```

