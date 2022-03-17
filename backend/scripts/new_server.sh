#!/bin/bash

# Install all deps
sudo apt update
sudo touch /etc/apt/sources.list.d/backports.list
deb http://ftp.debian.org/debian stretch-backports main
sudo apt install -y python-certbot-nginx -t stretch-backports
sudo apt install -y redis-server nginx postgresql dbus

# Disable default nginx config
sudo mv /etc/nginx/conf.d/default.conf /etc/nginx/conf.d/default.conf.disabled
sudo mv /etc/nginx/sites-enabled/default.conf /etc/nginx/sites-enabled/default.conf.disabled
# Enable our custom nginx config
sudo cp ./locationless.conf /etc/nginx/sites-enabled/locationless.conf

# Need to put the `service` binary on the path
export PATH=$PATH:/usr/sbin

sudo service enable nginx
sudo service start nginx
sudo service reload nginx

# Copy the diesel CLI to the working dir (so we can run migrations on deploy)
gsutil cp gs://locationless/diesel .
chmod 777 ./diesel

##########################
# CERT BOT
##########################

# sudo certbot --nginx

