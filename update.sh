#!/bin/sh
SNIPPETS_DIR=~/.local/share/nvim/snippets
if [ ! -d $SNIPPETS_DIR ]; then
  mkdir $SNIPPETS_DIR
fi

cargo snippet > $SNIPPETS_DIR/rust.snip
