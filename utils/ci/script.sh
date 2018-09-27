# Derived from https://github.com/japaric/trust

set -ex

main() {
    if [ ! -z $DISABLE_TESTS ]; then    # tests are disabled
        cross build --all --target $TARGET --release
        return
    fi

    cross test --target $TARGET --all
    cross test --target $TARGET --package rand_pcg --features serde1
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
