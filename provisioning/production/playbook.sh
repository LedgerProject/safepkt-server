#!/bin/bash

export SCHEME_HOST="http://localhost:3000"

function safepkt_server_00_list_steps() {
    curl "${SCHEME_HOST}"/steps | jq
}

function safepkt_server_01_upload_source() {
    curl --request POST \
    --url "${SCHEME_HOST}"/source \
    --header 'Content-Type: application/json' \
    --data '{"source":"dXNlIHZlcmlmaWNhdGlvbl9hbm5vdGF0aW9uczo6cHJlbHVkZTo6KjsKCmZuIG1haW4oKSB7CiAgICBsZXQgYSA9IHUzMjo6YWJzdHJhY3RfdmFsdWUoKTsKICAgIGxldCBiID0gdTMyOjphYnN0cmFjdF92YWx1ZSgpOwogICAgdmVyaWZpZXI6OmFzc3VtZSgxIDw9IGEgJiYgYSA8PSAxMDAwKTsKICAgIHZlcmlmaWVyOjphc3N1bWUoMSA8PSBiICYmIGIgPD0gMTAwMCk7CiAgICBpZiB2ZXJpZmllcjo6aXNfcmVwbGF5KCkgewogICAgICAgIGVwcmludGxuISgiVGVzdCB2YWx1ZXM6IGEgPSB7fSwgYiA9IHt9IiwgYSwgYik7CiAgICB9CiAgICBsZXQgciA9IGEgKiBiOwogICAgdmVyaWZpZXI6OmFzc2VydCEoMSA8PSByICYmIHIgPD0gMTAwMDAwMCk7Cn0K"}'
}

function safepkt_server_02_llvm_bitcode_generation() {
    curl -XPOST "${SCHEME_HOST}"/llvm-bitcode-generation/4e280b3d47 | jq
}

function safepkt_server_03_get_llvm_bitcode_generation_progress() {
    curl -XGET "${SCHEME_HOST}"/llvm-bitcode-generation/4e280b3d47/progress | jq
}

function safepkt_server_04_get_llvm_bitcode_generation_report() {
    curl -XGET "${SCHEME_HOST}"/llvm-bitcode-generation/4e280b3d47/report  | jq
}

function safepkt_server_05_symbolic_execution() {
    curl -XPOST "${SCHEME_HOST}"/symbolic-execution/4e280b3d47 | jq
}

function safepkt_server_06_get_symbolic_execution_progress() {
    curl -XGET "${SCHEME_HOST}"/symbolic-execution/4e280b3d47/progress | jq
}

function safepkt_server_07_get_symbolic_execution_report() {
    curl -XGET "${SCHEME_HOST}"/symbolic-execution/4e280b3d47/report  | jq
}

