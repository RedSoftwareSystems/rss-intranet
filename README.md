# CORE  DAO ##

## Integration tests ##
To run integration tests you need to use a postgres database. The fastest way is to obtain it from docker.

Use the following command:\
`docker run --name 'pgactix' -p 5433:5432 -e POSTGRES_PASSWORD=pgactix -e POSTGRES_USER=pgactix -d postgres`\

Once the db has started, load the script `./test_resources/pgactix.dump` with this command:\
`docker run -ti --rm --link pgactix:pgactix -e PGPASSWORD=pgactix -v $(pwd)/test_resources:/init postgres /bin/bash`\
From inside the container run this command to create tables and upload test data:\
`psql -h pgactix -U pgactix < /init/pgactix.dump`