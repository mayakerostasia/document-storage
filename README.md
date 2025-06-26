# Document Storage
by Maya Kerostasia  

[![Rust Check/Test/Fmt](https://github.com/mayakerostasia/document-storage/actions/workflows/test.yaml/badge.svg)](https://github.com/mayakerostasia/document-storage/actions/workflows/test.yaml)
[![stable-beta-nightly](https://github.com/mayakerostasia/document-storage/actions/workflows/msvc.yaml/badge.svg)](https://github.com/mayakerostasia/document-storage/actions/workflows/msvc.yaml)

## tee-doc
A JSON object 'tee' program that 

1. takes json as input, 
2. Stores it in a document database ( like quickwit ) 
3. Then returns it as it was input

## Help
```
This is a CLI for Piping JSON data to quickwit. 
It allows you to take output from another command and uploads it to quickwit.


Usage: tee-doc.exe [COMMAND]

Commands:
  upload  Quickwit Upload
  search  Document Search
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Example
![tee-doc run example](assets/tee-doc-run.png)
