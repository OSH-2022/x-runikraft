#!/bin/bash
make_type=`cat ${CONFIG_DIR}/.features2`
features=`cat ${CONFIG_DIR}/.features1`
if [ $make_type = "release" ]
then
    RELEASE=--release
elif [ $make_type = "debug" ]
then
    RELEASE=""
else
    echo "Unknown build type, expect release/debug."
    exit 1
fi

#cd example/sudoku && env RUNIKRAFT_CONFIG_FILE="${CONFIG_DIR}/config.rs" RUSTFLAGS="-Clink-arg=-T${SRC_ROOT_DIR}/linker.ld --cfg __alloc_error_handler --cfg __runikraft_custom_config --extern __alloc_error_handler=${MAKE_ROOT_DIR}/liballoc_error_handler.rlib" cargo build $RELEASE $features
cd example/sudoku && env RUSTFLAGS="-Clink-arg=-T${SRC_ROOT_DIR}/linker.ld --cfg __alloc_error_handler --extern __alloc_error_handler=${MAKE_ROOT_DIR}/liballoc_error_handler.rlib" cargo build $RELEASE $features
