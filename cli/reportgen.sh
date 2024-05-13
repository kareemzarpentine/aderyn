#!/bin/bash

cd tests/contract-playground
rm -r out
rm -r sample_out
forge build --ast
FOUNDRY_PROFILE=sample forge build --ast
cd ../../

#### MARKDOWN REPORTS ######

# Basic report.md 
cargo run -- ./tests/contract-playground -o ./reports/report.md --skip-update-check &

# Basic report.profile.md 
FOUNDRY_PROFILE=sample cargo run -- -o ./reports/report.sample_profile.md ./tests/contract-playground --skip-update-check &

# Create report-config.md based on config file
cargo run -- --config-file ./tests/aderyn.config.json -o ./reports/report-config.md ./tests/contract-playground/ --skip-update-check  &

##### JSON REPORTS ########

# Basic report.json
cargo run -- ./tests/contract-playground -o ./reports/report.json --skip-update-check &

# Create report-config.json based on config file
cargo run -- --config-file ./tests/aderyn.config.json -o ./reports/report-config.json ./tests/contract-playground/ --skip-update-check &

##### SARIF REPORTS ########

# Basic report.sarif
cargo run -- ./tests/contract-playground -o ./reports/report.sarif --skip-update-check &

wait
