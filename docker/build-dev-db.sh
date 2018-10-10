#!/bin/sh
docker run --name rss-intranet -p 5433:5432 -e POSTGRES_PASSWORD=postgres -d 