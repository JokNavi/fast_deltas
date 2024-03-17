#!/bin/bash

pgzip() {
[[ -f "$1.gz" ]] && rm "$1.gz"
[[ -f $1 ]] && gzip -k $1
}

alias gzip-txt="pgzip \"test_files/txt/patch.dpatch\""
alias gzip-exe="pgzip \"test_files/exe/patch.dpatch\""