# spades-runner

![spades-runner](https://github.com/hhandika/spades-runner/workflows/Tests/badge.svg)
[![Build Status](https://www.travis-ci.com/hhandika/spades-runner.svg?branch=main)](https://www.travis-ci.com/hhandika/spades-runner)

`spades-runner` is a command-line application to batch assembly next-gen sequencing reads using SPAdes. It aims to simplify batch assembly process with minimal to no-configuration file.

## Table of Contents

- [Quick Start](#quick-start)
- [Installation](#installation)
- [Usages](#usages)
  - [Command Structure](#command-structure)
  - [Auto Assembly](#auto-assembly)
  - [Manual Assembly with a Config File](#manual-assembly-with-a-config-file)
  - [Threading](#threading)
- [State of Code](#state-of-code)

## Quick Start

If your folder structure similar to [fastp-runner](https://github.com/hhandika/fastp-runner) or [phyluce](https://phyluce.readthedocs.io/en/latest/) or use either program to clean your sequencing reads, you can use the auto detection command. The app will detect your folder structure. Therefore, there is no need to use a configuration file.

```
spr auto -d [you-clean-read-folder]
```

An option to use a configuration file is also available. You can use a two-column csv:

| Samples         | Path                                       |
| --------------- | ------------------------------------------ |
| some_species    | clean_reads/some_species/trimmed_reads/    |
| another_species | clean_reads/another_species/trimmed_reads/ |

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

You can check if the app correctly detect your reads using the `dry-run` option:

```
spr auto -d [your-clean-read-folder] --dry
```

or

```
spr assembly -i [path-to-your-config-file] --dry
```

By default, the app passes `--careful` options to SPAdes. The full command is equal to running SPAdes using this command:

```
spades --pe1-1 [path-to-read1] --pe1-2 [path-to-read2] -o [target-output-dir] --careful
```

It will add `--pe1-s [path-to-singleton/unpaired-read]` if the app detects a singleton read in your sample directory.

You can also specify the number of threads by passing `-t` or `--threads` option:

```
spr auto -d [your-clean-read-folder] -t [number-of-threads]
```

or if you use a config file:

```
spr assembly -i [path-to-your-config-file] -t [number-of-threads]
```

Other SPAdes parameter is available by using `--opts` option. The given parameters should be in a qoute and starts with `params=`. For example, here we define max memory size to 16 gb. The program will override the careful option used in the default settings. Hence, we will need to pass it again if we want to use it.

```
spr auto -d clean_reads/ --opts "params=--careful -m 16"
```

The app won't check the correctness of the parameters. Instead, it will let SPAdes checking them. This way it gives user flexibility to pass any SPAdes cli parameters available for pair-end reads.

For more options:

```
spr --help
```

Options for specific subcommands:

```
spr <SUBCOMMAND> --help
```

```
spr auto --help
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

If it shows `[NOT FOUND]`, then install SPAdes. You can find the instruction to install SPAdes [here](https://cab.spbu.ru/software/spades/). After installation, you should `check` again if the app can recognize SPAdes installation.

## Usages

### Command Structure

All available sub-command options:

```
USAGE:
    spr <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    assembly    Runs SPAdes using a config file
    auto        Auto find clean reads and assembly them
    check       Checks if SPAdes is installed
    clean       Cleans unused SPAdes files.
    help        Prints this message or the help of the given subcommand(s)
```

Options available for auto assembly:

```
USAGE:
    spr auto [FLAGS] [OPTIONS] --dir <CLEAN-READ DIR>

FLAGS:
        --dry        Checks if the program can find the correct files
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dir <CLEAN-READ DIR>      Inputs a directory for auto search
        --opts <OPTIONAL PARAMS>    Sets optional SPAdes params
    -o, --output <OUTPUT DIR>       Specifies output folders
    -s, --specify <DIR NAME>        Specifies clean read directory names [default: trimmed]
    -t, --threads <THREAD-NUM>      Sets number of threads
```

Options available for assembly with a config file:

```
USAGE:
    spr assembly [FLAGS] [OPTIONS]

FLAGS:
        --dry        Checks if the program detect the correct files
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <INPUT>             Inputs a config file
        --opts <OPTIONAL PARAMS>    Sets optional SPAdes params
    -o, --output <OUTPUT DIR>       Specifies output folders
    -t, --threads <THREAD-NUM>      Sets number of threads
```

### Auto Assembly

More update coming soon...

### Manual Assembly with a Config File

More update coming soon...

### Threading

Threading options is available for auto and manual assembly using the commands `-t` or `--threads`. If you don't know the number of threads available in your system, you can use `spr check` to find it out. The result is as below:

```{Bash}
System Information
Operating system        : openSUSE Tumbleweed 20210210
Kernel version          : 5.10.12-1-default
Available cores         : 12
Available threads       : 24
Total RAM               : 31 Gb

Dependencies:
[OK]    SPAdes genome assembler v3.15.1
```

Then, use the flag -t to specify the threads. It also works with the manual assembly.

```{Bash}
spr auto -d [your-clean-read-folder] -t [number-of-threads]
```

For example:

```{Bash}
spr auto -d /clean_reads -t 8
```

## State of Code

The program is still under-development. However, it is stable and tested. If you find any bugs or if you want to request a feature, please open an [issue for this repo](https://github.com/hhandika/spades-runner/issues).

```

```
