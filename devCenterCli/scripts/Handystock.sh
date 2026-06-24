#!/bin/bash

dev-up

gnome-terminal -- bash -c "
cd /var/www/html/HandyStock
php artisan serve
exec bash
"

gnome-terminal -- bash -ic "
cd /home/bartoloni/Documentos/HandyStock

source ~/.nvm/nvm.sh
nvm use 24

npm run dev

exec bash
"

google-chrome http://localhost:8000 &