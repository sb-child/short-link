#!/bin/sh

cd migration || exit
cargo r -- up -u postgres://shortlnk:shortlnk@127.0.0.1/shortlnk_db || exit
cd .. || exit

sea-orm-cli generate entity --database-url postgres://shortlnk:shortlnk@127.0.0.1/shortlnk_db -o entity/src/entities
