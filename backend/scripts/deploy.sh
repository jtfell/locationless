#!/bin/bash
set -x
set -e

cargo test

COMPUTE_INSTANCE=locationless-app-micro
gcloud config configurations activate locationless

# Build with CI and download to local
# gcloud builds submit --config cloudbuild.yml .
mkdir -p target/remote
gsutil cp gs://locationless/backend ./target/remote/backend

# Copy everything to server
gcloud compute scp target/remote/backend $COMPUTE_INSTANCE:~/backend_new
gcloud compute scp ops/locationless.conf $COMPUTE_INSTANCE:~/locationless.conf
gcloud compute scp ops/locationless-app.service $COMPUTE_INSTANCE:~/locationless-app.service
gcloud compute scp .prod.env $COMPUTE_INSTANCE:~/.env

# Add any migrations
gcloud compute scp Cargo.toml $COMPUTE_INSTANCE:~/Cargo.toml
gcloud compute scp --recurse migrations $COMPUTE_INSTANCE:~/

gcloud compute scp scripts/run_on_server.sh $COMPUTE_INSTANCE:~/run_on_server.sh

gcloud compute ssh $COMPUTE_INSTANCE --command 'sudo chmod 777 run_on_server.sh'
gcloud compute ssh $COMPUTE_INSTANCE --command 'sudo ./run_on_server.sh'
