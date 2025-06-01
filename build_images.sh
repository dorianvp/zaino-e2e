#! /usr/bin/env bash

set -ex

for repo in \
  "git@github.com:zingolabs/zaino.git zaino . zainod:test Dockerfile" \
  "git@github.com:ZcashFoundation/zebra.git zebra . zebrad:test docker/Dockerfile"
do
  set -- $repo
  url="$1"
  dir="$2"
  build_dir="$3"
  image_tag="$4"
  dockerfile="$5"

  if [ ! -d "$dir" ]; then
    git clone "$url"
  elif [ -d "$dir/.git" ]; then
    pushd "$dir"
    git pull
    popd
  else
    echo "Directory $dir exists but is not a git repo, skipping."
    continue
  fi

  pushd "$dir"
  docker build -f "$dockerfile" -t "$image_tag" "$build_dir"
  popd
done
