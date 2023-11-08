# Dusty Study - an anagram solver

## Initial setup

To install everything you need and confirm that the app builds, run:

```
./build.sh
```

This is the script that Netlify runs to build the app.

## Running the app locally

To serve the app using Trunk, run:

```
./serve.sh
```

Every time you make a change, Trunk will recompile and live-reload the page.

## Deploying to Netlify

Just push `main` to GitHub.

## Repo overview

### Frontend

The frontend app is implemented with [Yew](https://yew.rs). The relevant files are:

* `./Cargo.toml`
* `./index.html`
* `./src/`

The Yew app is built using Trunk. The output files are written to the `dist/`
directory.

### Backend

The backend is a single Netlify function implemented in Rust.

It accepts a query string as input, looks up a list of matching anagrams in its
database, and returns them in its response.

The relevant files are:

* `netlify/functions/solve/`

### Dataset

The `dataset/` directory contains a Bash script and a Rust script for building
the database of anagrams.

The scripts take a
[kaikki.org](https://kaikki.org/dictionary/English/index.html) dictionary file
as input and transform them into an anagram dataset. The result is written as a
JSON file inside the backend's `src` directory.

To build the dataset:

1. Run `./download-dictionary.sh` inside the `dataset` directory. This will
   download the dictionary file. It is not stored in git because it is > 1 GB.

2. Run `./extract-words.sh` to do some preprocessing.

3. Run `cargo run --release` to generate the anagram dataset.
