#!/bin/bash

# Run any DB migrations that have been added
sudo -u postgres ./diesel migration run

# Restart the app (tiny bit of downtime...)
# cp ~/locationless-app.service /etc/systemd/system
sudo systemctl daemon-reload
sudo service locationless-app stop
mv backend backend_old
cp backend_new backend
chmod 7777 backend
sudo service locationless-app start
