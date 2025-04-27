# Oxford Dictionary Lib
A small library providing function for scrapping definitions, examples and search results on [Oxford Learner's Dictionary](https://www.oxfordlearnersdictionaries.com/).

## Requirements
1. `rustc` of version 1.80.0 and greater

## Dependencies
- `loggit`
- `reqwest`
- `scraper`
- `tokio`

## Usage
Add the next code you your `Cargo.toml`

```toml
[dependencies]
oxford_dictionary_lib = { git = "https://github.com/DobbiKov/oxford-dictionary-lib.git" }
```

Example:
```rs
use oxford_dictionary_lib::{search_dictionary, ParseLinkResult};

let r = search_dictionary("convinient").await.unwrap();
```

## Implementations
1. [Oxford Dictionary Telegram Bot GitHub](https://github.com/DobbiKov/oxford_dictionary_bot/) 
