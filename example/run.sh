cargo run -- \
    --program example/protoc-gen-ts/runner \
    --output example/protoc-gen-ts/test.tap \
    --runner-stderr example/protoc-gen-ts/test.log \
    --runner-env NO_COLOR=1 \
    --json-stats example/protoc-gen-ts/stats.json $@