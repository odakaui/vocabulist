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

cp target/$TARGET/release/$BINARY_NAME $stage/

cd $stage

# if the commit has a tag add it to the filename
if [ -z "$var" ]
then
    echo "Draft Release"
    tar czf "${src}/${NAME}-${TARGET}.tar.gz" ./*
else
    tar czf "${src}/${NAME}-${TRAVIS_TAG}-${TARGET}.tar.gz" ./*
fi

cd $src

rm -rf $stage
