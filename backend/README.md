# Locationless

An app for figuring out who is where, when.

## Setup
- `cargo install diesel_cli`
- `brew services start redis`
- `brew services start postgresql`
- `cp .env.example .env`
- `diesel setup --database-url='postgres://localhost/locationless'` (Make sure it matches the `.env`)
- `diesel migration run` to create the user database tables
- `psql locationless < data/seed.sql` to add a user with creds:
- `cargo run` 

## Deployment

- `gcloud config configurations activate locationless`
- `gcloud compute ssh locationless-backend-micro`

## New Server Setup

- Create a new VM instance on GCP Compute Engine (Debian-based)
- Change the `COMPUTE_INSTANCE` env var in `deploy.sh` and `new.sh` scripts
- Run `./scripts/new.sh`
- Run `./scripts/deploy.sh`

Bit of a chicken & egg situtation here... Can't run certbot without DNS pointed
at the new server, but can't switch the DNS over to a server with no HTTPS cert.

- Set the external (static) IP address to point at the new instance
- ssh into the new instance
- Run `sudo certbot --nginx` and follow the prompts


## Telegram Bot

- Reserved:
https://t.me/LocationlessBot
https://t.me/LocationlessDevBot
