# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin custom_rich_status --target $TARGET --release -- -C lto

    # Eating copy error on matching either Linux or Windows binary
    cp target/$TARGET/release/custom_rich_status{,.exe} $stage/ || true
    cp config.yml $stage/
    cp -r presets/ $stage/

    cp LICENCE $stage/
    cp README.md $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
