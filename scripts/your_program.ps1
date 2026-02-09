# PowerShell script equivalent to the provided bash script

# Use this script to run your program LOCALLY.
#
# Note: Changing this script WILL NOT affect how CodeCrafters runs your program.
#
# Learn more: https://codecrafters.io/program-interface

# Exit early if any commands fail
$ErrorActionPreference = "Stop"

# Copied from .codecrafters/compile.sh
#
# - Edit this to change how your program compiles locally
# - Edit .codecrafters/compile.sh to change how your program compiles remotely
cargo build `
    --quiet `
    --release `
    --target-dir="/tmp/lox-rs" `
    --manifest-path ./interpreter/Cargo.toml

# Copied from .codecrafters/run.sh
#
# - Edit this to change how your program runs locally
# - Edit .codecrafters/run.sh to change how your program runs remotely
& "/tmp/lox-rs/release/lox-rs" $args