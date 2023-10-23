# Shuffle Sort

Wacky woohoo sorting problem where buffering matters more than sorting.

## Input

- A file containing 1M words
  - Separated by newline
  - 5~20 characters
  - Lowercase `[a-z]`

- Shuffled sorting order of alphabets
  1. Swap each element at index `2K` with the element at index `2K + 1`.\
    e.g. `[1, 2, 3, 4]` → `[2, 1, 4, 3]`
  2. Swap the first half with the last half of the array.\
    e.g. `[1, 2, 3, 4]` → `[3, 4, 1, 2]`

In short, sort by `"mporqtsvuxwzybadcfehgjilkn"` instead of `"abcdefghijklmnopqrstuvwxyz"`.

## Output

- Sort the words based on the given order
- Print the words to stdout, with each word separated by newline
