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
esac

cp target/$TARGET/release/vocabulst_rs $stage/

cd $stage

# if the commit has a tag add it to the filename
if [ -z "$var" ]
then
    tar czf "${src}/vocabulist-${TARGET}.tar.gz"
else
    tar czf "${src}/vocabulist-${TRAVIS_TAG}-${TARGET}.tar.gz"
fi

cd $src

rm -rf $stage
