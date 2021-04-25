#!/usr/env/bin sh

TARGET_PATH="$1"

echo id
echo rm -Rf $TARGET_PATH/*
echo cp build/* $TARGET_PATH/
