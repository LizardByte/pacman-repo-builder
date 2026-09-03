#! /bin/bash
set -o errexit -o pipefail -o nounset
cd ./flatten
sha256sum -- * >../sha256sum.txt
sha512sum -- * >../sha512sum.txt
