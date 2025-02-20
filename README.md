CRX2RNX
=======

[![Rust](https://github.com/rtk-rs/crx2rnx/actions/workflows/rust.yml/badge.svg)](https://github.com/rtk-rs/crx2rnx/actions/workflows/rust.yml)
[![Rust](https://github.com/rtk-rs/crx2rnx/actions/workflows/daily.yml/badge.svg)](https://github.com/rtk-rs/crx2rnx/actions/workflows/daily.yml)
[![crates.io](https://img.shields.io/crates/v/crx2rnx.svg)](https://crates.io/crates/crx2rnx)

[![License](https://img.shields.io/badge/license-MPL_2.0-orange?style=for-the-badge&logo=mozilla)](https://github.com/rtk-rs/qc-traits/blob/main/LICENSE)

`crx2rnx` is a small command line utility to decompress
your CRINEX (Compact RINEX) files into readable RINEX. It aims at becoming
a modern replacement of the historical tool.

This tool is based on the [GeoRust/RINEX parser](https://github.com/georust/rinex).

## Download the tool

You can download the latest version from [the release portal](https://github.com/rtk-rs/crx2rnx/releases)

## Install from Cargo

You can directly install the tool from Cargo with internet access:

```bash
cargo install crx2rnx
```

## Build from sources

Download the version you are interested in:

```bash
git clone https://github.com/rtk-rs/crx2rnx
```

And build it using cargo:

```bash
cargo build --all-features -r
```

Getting started
===============

The tool expects one input file that needs to be a valid CRINEX file:

```bash
crx2rnx AJAC3550.21D
Decompressed AJAC3550.21O
```

By default the tool lets you know what the output file is (`stdout`).  

If that bothers you, simply use `-q` (quiet option): 

```bash
crx2rnx -q AJAC3550.21D
```

CRINEX V1/V3
============

Both revisions are supported by this tool.

The tool preserves the input format by default, so standardized V3 filenames will produce a standardized file name:

```bash
crx2rnx ACOR00ESP_R_20213550000_01D_30S_MO.crx
Decompressed ACOR00ESP_R_20213550000_01D_30S_MO.rnx
```

We have one option that let's you convert a V3 format to V2 directly:

```bash
crx2rnx -s ACOR00ESP_R_20213550000_01D_30S_MO.crx
Decompressed ACOR3550.21O
```

## Gzip files

The tool supports both I/O gzip compressed data. You can decompress
your Gzip CRINEX files natively:

```bash
crx2rnx 
```

The tool preserves the input format, by default, this will generate a new Gzip'ed file.  
You can use `--unzip` to specify you want to gzip decompress at the same time.  

You can use this tool to Gzip compress at the same time, in this example we 
decompress a "plain" CRINEX into a gzip compressed RINEX:

```bash

```

Finally, when defining a custom RINEX name, you have two options

## Licensing

This application is part of the [RTK-rs framework](https://github.com/rtk-rs) which
is delivered under the [Mozilla V2 Public](https://www.mozilla.org/en-US/MPL/2.0) license.
