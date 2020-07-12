#!/usr/bin/env bash

set -ex

src="$(pwd)"

case $TRAVIS_OS_NAME in
    linux)
        stage=$(mktemp -d)
        ;;
    osx)
        stage=$(mktemp -d -t tmp)
        ;;
    windows)
        stage=$(mktemp -d)
        ;;
esac

mkdir $stage/$NAME

cp target/$TARGET/release/$BINARY_NAME $stage/$NAME
cp jmdict.db $stage/$NAME
cp ACKNOWLEDGEMENTS.md $stage/$NAME
cp LICENSE.md $stage/$NAME

cd $stage

# if the commit has a tag add it to the filename
if [ -z "$TRAVIS_TAG" ]
then
    echo "Draft Release"
    tar czf "${src}/${NAME}-${TARGET}.tar.gz" $NAME
else
    tar czf "${src}/${NAME}-${TRAVIS_TAG}-${TARGET}.tar.gz" $NAME
fi

cd $src

rm -rf $stage
