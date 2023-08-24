#! /bin/sh

# See also `build.rs` script that copys generated files (*.a) into
# `cargo`'s target directory.


# The repo. we want to download from.
#
ICEORYX_REPO_NAME="iceoryx"
ICEORYX_REPO_URL="https://github.com/eclipse-iceoryx/iceoryx"

CYCLONE_DDS_REPO_NAME="cyclonedds"
CYCLONE_DDS_REPO_URL="https://github.com/eclipse-cyclonedds/cyclonedds"


THIS_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )


# Setting up a temporal directory
#
#
if [[ -z "${OUT_DIR}" ]] || [ -n "${IN_TMP}" ] ; then
    # Make one in the current directory.
    mkdir -p ./tmp
    TMP_DIR=${THIS_DIR}/tmp
else
    # Use one in the Cargo's.
    TMP_DIR=${OUT_DIR}
fi

cd ${TMP_DIR}

# Download and build Iceoryx.
#
#
git clone ${ICEORYX_REPO_URL} --single-branch
cd ${ICEORYX_REPO_NAME}

cmake -Bbuild -Hiceoryx_meta -DCMAKE_INSTALL_PREFIX=${TMP_DIR}/usr/local/
cmake --build build
cmake --build build --target install

cd -


# Download and build Cyclone DDS
#
#

git clone ${CYCLONE_DDS_REPO_URL} --single-branch

cd ${CYCLONE_DDS_REPO_NAME}

mkdir build

cd build

cmake -DCMAKE_INSTALL_PREFIX=${TMP_DIR}/usr/local/ ..
cmake --build .
cmake --build . --target install
cd -
