# Derived from https://github.com/japaric/trust

set -ex

main() {
    if [ ! -z $DISABLE_TESTS ]; then    # tests are disabled
        cross build --target $TARGET --release
        return
    fi

    if [ ! -z $NIGHTLY ]; then  # have nightly Rust
        cross test --target $TARGET
    else    # have stable Rust
        cross test --target $TARGET
    fi
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
