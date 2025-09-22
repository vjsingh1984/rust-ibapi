
# Generate and save coverage report using tarpaulin
cover:
    cargo tarpaulin -o html
    echo "coverage report saved to tarpaulin-report.html"

# Tags repo with specified version
tag VERSION:
    echo "Tagging repo with version {{VERSION}}"
    git tag {{VERSION}} -m "Version {{VERSION}}"
    git push origin {{VERSION}}

# Lists all available versions
versions:
    @git tag

# Run tests across feature configurations
test:
    @echo "Running default (async) tests..."
    cargo test
    @echo ""
    @echo "Running async + sync tests..."
    cargo test --features sync
    @echo ""
    @echo "Running sync-only tests..."
    cargo test --no-default-features --features sync
