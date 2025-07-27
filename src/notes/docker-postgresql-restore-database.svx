---
title: "Restore PostgreSQL database running in Docker Container"
description: "Short guide on restoring your PostgreSQL database instance running in Docker"
categories: [pg_restore, dump, restore, docker, postgresql, psql, sql]
icon: docker
date: 2021-11-13
preview_image_url: "https://images.unsplash.com/photo-1589532768434-a92c95dad7cb?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=870&q=80"
published: true
---

When you are working on a feature that involves implementing new queries to
the database, you may want to use a local environment to run your SQL queries
instead of running these commands against the production instance.

This is a good idea if this database or the dump in question doesn't include
any sensitive records.

> Avoid dumping your database if sensitive data may be included in such dump, **it's a big deal to leak sensitive data**.

If you are not sure on what does _sensitive data_ means, here's some examples:

- Credit Card details
- Sevice Tokens
- Passwords (Even hashes)
- Phone Numbers
- Email Addresses
- Physhical Addresses

## Running a PostgreSQL instance using Docker

Open your terminal and paste the following docker command:

```bash
docker run --name my_fav_pg_instance \
  -e POSTGRES_PASSWORD=secret \
  -d postgres
```

Here we are pulling the `postgres` image on it's latest release
(`postgres:latest`). Then run `docker ps` to ensure the instance is running
as expected:

```bash
docker ps

# Expect similar output
CONTAINER ID   IMAGE      COMMAND                  CREATED              STATUS              PORTS      NAMES
0a6dd23bac73   postgres   "docker-entrypoint.s…"   About a minute ago   Up About a minute   5432/tcp   my_fav_pg_instance

```

This means our PostgreSQL image was builded and is now running on a container
with the ID: `0a6dd23bac73`, the PostgreSQL instance is reachable on port `5432`.

## Recover the database using an existing _dump_ file

First let's execute a `CREATE DATABASE` query in our _dockerized_ PostgreSQL
instance, this is the database we are going to use to run our "Recovery" or
"Restore" process, I'm calling it _back_from_dump_.

```bash
docker exec -e PGPASSWORD=secret -i 0a6dd23bac73 psql -U postgres -d postgres -p 5432 -c 'CREATE DATABASE back_from_dump'
```

Then you must execute the `pg_restore` command through docker as well:

```bash
docker exec -e PGPASSWORD=secret -i 0a6dd23bac73 pg_restore -U postgres -p 5432 -v -d postgres < ~/Files/back_from_dump.dump

```

If it happens that you have ACL or Owner features in your dump, you can skip
them using the `--no-owner`, `--no-acl` options.

```bash
docker exec -e PGPASSWORD=secret -i 0a6dd23bac73 pg_restore -U postgres -p 5432 -v -d postgres --no-owner --no-acl < ~/Files/back_from_dump.dump
```

## Conclusion

To summarize, you are able to execute a dump on a PostgreSQL instance everywhere
using the Dockerized environment, make sure your dump files are safe and
sensitive data free.

If you find any issues with this note, please don't hesitate to open a PR or
issue in my [GitHub](https://github.com/LeoBorai/leoborai.com)!
