#!/bin/bash
set -x
set -e

COMPUTE_INSTANCE=locationless-app-micro
gcloud config configurations activate locationless

gcloud compute scp scripts/new_server.sh $COMPUTE_INSTANCE:~/new_server.sh
gcloud compute scp ops/locationless.conf $COMPUTE_INSTANCE:~/locationless.conf
gcloud compute ssh $COMPUTE_INSTANCE --command 'sudo chmod 777 new_server.sh'
gcloud compute ssh $COMPUTE_INSTANCE --command 'sudo ./new_server.sh'
