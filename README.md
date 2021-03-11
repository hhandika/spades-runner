# spades-runner
![spades-runner](https://github.com/hhandika/spades-runner/workflows/Tests/badge.svg)
[![Build Status](https://www.travis-ci.com/hhandika/spades-runner.svg?branch=main)](https://www.travis-ci.com/hhandika/spades-runner)

`spades-runner` is command-line application to batch assembly next-gen sequencing reads using SPAdes. It aims to simplify batch assembly process with minimal to no-configuration file. If your folder structure similar to fastp-runner or phyluce or use either program to clean your sequencing reads, you can use the auto function. There is no need to use a configuration file. spade-runner will detect your folder structure use similar name from your clean_read folders.

```
spr auto -d [you-clean-read-folder]
```

Option to add configuration file is also possible. You can use a two columns csv:

|Species        | Folder                                    |
|---------------|-------------------------------------------|
|some_species   |clean_reads/some_species/trimmed_reads/    |
|another_species|clean_reads/another_species/trimmed_reads/ |

Or using ini format:

```
[species]
some_species:clean_reads/some_species/trimmed_reads/
another_species:clean_reads/another_species/trimmed_reads/
```

You can check if the app correctly detect your reads using the `dry run` option:

```
spr auto -d [your-clean-read-folder] --dry
```

## Installation
Installation is similar to simple-qc. For now, please follow the instruction in simple-qc. Then install spades. You could also check if you already have SPAdes installed in your system:

```
spr check
```
It will shows your system information and spades version you have. This is example of the result from my computer:

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