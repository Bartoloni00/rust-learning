#!/bin/bash

dev-up

gnome-terminal -- bash -c "
cd /var/www/html/HandyStock
php artisan serve
exec bash
"

gnome-terminal -- bash -c "
cd /home/bartoloni/Documentos/HandyStock
npm run dev
exec bash
"

google-chrome http://localhost:8000 &