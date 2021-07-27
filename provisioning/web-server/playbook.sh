#!/bin/bash

export SCHEME_HOST="http://localhost:3000"

function safepkt_backend_00_list_steps() {
    curl --no-progress-meter "${SCHEME_HOST}"/steps | jq
}

function safepkt_backend_01_upload_source() {
    curl --no-progress-meter --request POST \
        --url "${SCHEME_HOST}"/source \
        --header 'Content-Type: application/json' \
        --data '{"source":"dXNlIHZlcmlmaWNhdGlvbl9hbm5vdGF0aW9uczo6cHJlbHVkZTo6KjsKCmZuIG1haW4oKSB7CiAgICBsZXQgYSA9IHUzMjo6YWJzdHJhY3RfdmFsdWUoKTsKICAgIGxldCBiID0gdTMyOjphYnN0cmFjdF92YWx1ZSgpOwogICAgdmVyaWZpZXI6OmFzc3VtZSgxIDw9IGEgJiYgYSA8PSAxMDAwKTsKICAgIHZlcmlmaWVyOjphc3N1bWUoMSA8PSBiICYmIGIgPD0gMTAwMCk7CiAgICBpZiB2ZXJpZmllcjo6aXNfcmVwbGF5KCkgewogICAgICAgIGVwcmludGxuISgiVGVzdCB2YWx1ZXM6IGEgPSB7fSwgYiA9IHt9IiwgYSwgYik7CiAgICB9CiAgICBsZXQgciA9IGEgKiBiOwogICAgdmVyaWZpZXI6OmFzc2VydCEoMSA8PSByICYmIHIgPD0gMTAwMDAwMCk7Cn0K"}'
}

function safepkt_backend_02_llvm_bitcode_generation() {
    curl --no-progress-meter -XPOST "${SCHEME_HOST}"/llvm-bitcode-generation/4e280b3d47 | jq
}

function safepkt_backend_03_get_llvm_bitcode_generation_progress() {
    curl --no-progress-meter -XGET "${SCHEME_HOST}"/llvm-bitcode-generation/4e280b3d47/progress | jq
}

function safepkt_backend_04_get_llvm_bitcode_generation_report() {
    curl --no-progress-meter -XGET "${SCHEME_HOST}"/llvm-bitcode-generation/4e280b3d47/report | jq
}

function safepkt_backend_05_symbolic_execution() {
    curl --no-progress-meter -XPOST "${SCHEME_HOST}"/symbolic-execution/4e280b3d47 | jq
}

function safepkt_backend_06_get_symbolic_execution_progress() {
    curl --no-progress-meter -XGET "${SCHEME_HOST}"/symbolic-execution/4e280b3d47/progress | jq
}

function safepkt_backend_07_get_symbolic_execution_report() {
    curl --no-progress-meter -XGET "${SCHEME_HOST}"/symbolic-execution/4e280b3d47/report | jq
}
