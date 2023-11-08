#!/bin/bash
# Extract a list of suitable words from a kaikki.org JSON file.
# This JSON file is the result of processing the Wiktionary dump
# with the wiktextract tool: https://github.com/tatuylonen/wiktextract

# For each entry, we extract the word itself and any alternative forms, e.g. plurals.

# We filter out the following types of unhelpful words:
# - very short words (< 3 chars)
# - very long words (> 19 chars, including any spaces)
# - words with accented characters/diacritics
# - words with only uppercase characters (acronyms)

# We also normalise all words to uppercase. This is how we will want to display
# them, as the main use case for this tool is to help me with crosswords.
# It also makes it easier to spot duplicates that only differ in case (e.g. "hill" and "Hill")

echo -n "Extracting words..."
jq -r '.word,  .forms[]?.form' kaikki.org-dictionary-English.json \
  | LC_ALL=C grep '^[a-zA-Z ]\{3,19\}$' \
  | grep '[a-z]' \
  | tr 'a-z' 'A-Z' \
  > words.txt
echo "Done"

echo -n "Removing duplicates..."
sort -u words.txt > words-uniq.txt
echo "Done"

echo "Line count:"
wc -l words-uniq.txt

