#!/usr/bin/env bash

set -euo pipefail
set -x

if [[ "prod" != "${1-}" ]];
then
  echo "Loading polyfill libraries locally";
  compute-file-server-cli local --toml fastly.toml --name "polyfill-library" -- "./polyfill-libraries/";
else
  echo "Loading polyfill libraries remotely";
  compute-file-server-cli upload --token "$(fastly profile token --quiet polyfill)" --name "polyfill-library" -- "./polyfill-libraries/"
fi