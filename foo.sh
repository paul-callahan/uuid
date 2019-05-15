#!/usr/bin/env bash
set -x

  if [[ $(rustup default | cut -d ' ' -f 1) == 'nightly' ]]
  then
        cargo bench
        cargo bench --features 'serde'
        cargo bench --features 'v1'
        cargo bench --features 'v3'
        cargo bench --features 'v4'
        cargo bench --features 'v5'
        cargo bench --features 'slog'
        cargo bench --features 'serde std v1 v3 v4 v5'
  fi
