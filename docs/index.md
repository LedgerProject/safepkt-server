# Installation

Follow instructions from [Using KLEE on Coreutils](https://project-oak.github.io/rust-verification-tools/2021/07/14/coreutils.html) blog post
to set up Rust Verification Tools.

```shell
# A fork of Rust Verification Tool has additional commits
# offering the possibility 
# - to rely on the nightly build of rustc
# - having libsodium-dev package embedded in the base Docker image
git clone git@github.com:thierrymarianne/contrib-rust-verification-tools.git -b add-nightly-option rvt
```