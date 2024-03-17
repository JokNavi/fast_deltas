#!/bin/bash

single_path_gzip() {
    [[ -f "$1.gz" ]] && rm "$1.gz"
    [[ -f $1 ]] && gzip -k $1
}

single_path_xz() {
    [[ -f "$1.xz" ]] && rm "$1.xz"
    [[ -f $1 ]] && xz -e -k $1
}

compress_patch_gzip() {
    for arg in "$@"; do
        if [[ $arg == "--txt" ]]; then
            single_path_gzip "test_files/txt/patch.dpatch"
        fi
        if [[ $arg == "--exe" ]]; then
            single_path_gzip "test_files/exe/patch.dpatch"
        fi
    done
}

compress_patch_xz() {
    for arg in "$@"; do
        if [[ $arg == "--txt" ]]; then
            single_path_xz "test_files/txt/patch.dpatch"
        fi
        if [[ $arg == "--exe" ]]; then
            single_path_xz "test_files/exe/patch.dpatch"
        fi
    done
}

alias cgzip="compress_patch_gzip --txt --exe"
alias cxz="compress_patch_xz --txt --exe"