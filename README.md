# spades-runner
![spades-runner](https://github.com/hhandika/spades-runner/workflows/Tests/badge.svg)
[![Build Status](https://www.travis-ci.com/hhandika/spades-runner.svg?branch=main)](https://www.travis-ci.com/hhandika/spades-runner)

`spades-runner` is command-line application to batch assembly next-gen sequencing reads using SPAdes. It aims to simplify batch assembly process with minimal to no-configuration file. If your folder structure similar to fastp-runner or phyluce or use either program to clean your sequencing reads, you can use the auto detection command. The app will detect your folder structure. Therefore, there is no need to use a configuration file. 

```
spr auto -d [you-clean-read-folder]
```

Option to use a configuration file is also available. You can use a two-column csv:

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

If you use a configuration file, the command is as below:

```
spr assembly -i [path-to-your-config-file]
```


You can check if the app correctly detect your reads using the `dry run` option:

```
spr auto -d [your-clean-read-folder] --dry
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