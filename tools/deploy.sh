#!/usr/env/bin sh

TARGET_PATH="$1"

rm -Rf $TARGET_PATH/*
cp -R build/* $TARGET_PATH/
