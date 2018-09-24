# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=     \
          bin=target/$TARGET/release/custom_rich_status


    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    if [[ $TARGET = *"windows"* ]]
    then
        bin="$bin.exe"
    fi

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin custom_rich_status --target $TARGET --release -- -C lto

    cp $bin $stage/
    cp config.yml $stage/
    cp config.defaults.yml $stage/
    cp README.md $stage/
    cp LICENCE $stage/

    mkdir $stage/presets/
    cp presets/* $stage/presets/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src
}

main
