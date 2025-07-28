---
title: "Deploy Rust to Fly.io"
description: "Walkthrough on how to deploy a Rust application to Fly.io"
categories: [rust, postgresql, dns, flyio]
icon: rust
date: 2024-11-19
preview_image_url: "https://images.unsplash.com/photo-1506322526487-d8a94af63688?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&w=640"
published: true
---

## Creating the App and Deploying it

### Install Flyctl CLI

```bash
# macOS
brew install flyctl

# macOS or Linux (Unix)
curl -L https://fly.io/install.sh | sh

# Windows
pwsh -Command "iwr https://fly.io/install.ps1 -useb | iex"
```

Once installed, you can verify the installation by running `flyctl version`.

```
flyctl v0.3.38 linux/arm64 Commit: cafc23d0ce030323d9ca886790e47224493bebf6 BuildDate: 2024-11-15T19:51:38Z
```

> An alias for `flyctl` will also be created as `fly`.

### Authenticate on Fly.io

You can either use the `flyctl auth login` command to authenticate using the browser,
use the `FLY_API_TOKEN` environment variable or pass the token as a top-level option
to the CLI:

```bash
fly -t <TOKEN>
```

> Further steps uses the environment variable approach, so token is omitted from the examples.

### Create a new Fly App

To create a new Fly app, you can use the `flyctl apps create` command.

```bash
fly apps create <UNIQUE_APP_NAME>
```

> The name provided in this step should be unique for the Fly.io platform.

### Deploy your App

Once your app is created in Fly.io, you are ready to deploy your Rust application.

First create a `fly.toml`

```toml
app = "my-rust-app"

kill_signal = "SIGTERM"
kill_timeout = 5
processes = []
primary_region = "sea"

[build]
  image = "ghcr.io/estebanborai/my-rust-app:latest"

# https://fly.io/docs/reference/configuration/#the-env-variables-section
[env]
  RUST_LOG = "info"
  RUST_BACKTRACE = "1"

# https://fly.io/docs/reference/configuration/#the-http_service-section
[http_service]
  internal_port = 7878
  auto_stop_machines = true
  auto_start_machines = true
  min_machines_running = 0
  [http_service.concurrency]
    type = "requests"
    soft_limit = 200
    hard_limit = 250
```

Let's destructure the `fly.toml` file:

- `app`: The name of the Fly app you created in step 3, in this example it is `my-rust-app`.
- `build.image`: The Docker image that will be used to deploy the application, I have my image
in the GitHub Container Registry so I pull it from there using `docker pull` at the moment of
deploying, then I specify it so `flyctl` can use it.
- `env`: Environment variables that will be passed to the application.
- `http_service`: Configuration for the HTTP service that will be created in Fly.io.

Now you can deploy your application using the `flyctl deploy` command.

```bash
fly deploy --local-only -i ghcr.io/estebanborai/my-rust-app:latest
```

By default, the `flyctl deploy` command will build the Docker image in their servers and then
it will deploy it to the Fly.io platform. The `--local-only` flag tells `flyctl` to use the
image provided in the `fly.toml` file instead of building it.

> You can visit https://fly.io/apps/my-rust-app to check the status of your application.

## Provisioning a PostgreSQL Instance

Fly.io provides a managed PostgreSQL service that you can use to store your data, to create a
new PostgreSQL instance you can use the following command:

```bash
fly postgres create
```

You will be prompted some questions, I recommend using the same region as for your app, as stated
in Fly.io's documentation:

> Fly Postgres is a Fly app with flyctl sugar on top to help you bootstrap and manage a
> database cluster for your apps. It comes with most commonly used functionality (replication,
> failover, metrics, monitoring and daily snapshots).
>
> Source: https://fly.io/docs/postgres/

This means that the PostgreSQL instance will be created similarly to the previous app, but with
some API wrapper to simplify setup and maintenance.

> You can jump into this link to learn more: https://fly.io/docs/postgres/getting-started/what-you-should-know/.

Once your database is created, the Fly CLI will output the connection details, I recommend storing
these in a secure place, you will need to use the fly vpn command to connect to the database later.

You should see something like this:

```bash
Postgres cluster my-database created
  Username:    postgres
  Password:    <PASSWORD>
  Hostname:    <POSTGRES_APP_NAME>:internal
  Flycast:     <IPv6_ADDRESS>
  Proxy port:  <PROXY PORT>
  Postgres port:  <POSTGRES PORT>
  Connection string: <CONNECTION_STRING>

Save your credentials in a secure place -- you won't be able to see them again!
```

Once the instance is created, no databes will be available, you will need to create one,
follow on to learn how to create the database using the Fly CLI.

### Creating a Database

Fly CLI comes with a `fly postgres` command that you can use to interact with your
PostgreSQL using the `psql` shell.

Login to the PostgreSQL instance using the following command:

```bash
fly postgres connect -a <POSTGRES_APP_NAME>
```

> You will know you are on `psql` because the prompt will change to `postgres=#`.

This will connect to the PostgreSQL instance using the `psql` shell, you can now create
a database using:

```sql
CREATE DATABASE "<DATABASE_NAME>"
```

Use the command `\l` to list databases and make sure your database was created.

### Connecting to PostgreSQL from your Rust App

Connect to the PostgreSQL instance by attaching it to your Rust application using
the `attach` feature provided by Fly.io.

```bash
fly postgres attach <POSTGRES_APP_NAME> --app <APP_NAME>
```

This will add the `DATABASE_URL` secret to you `app` and will fill it up with the connection
string to the PostgreSQL instance.

Once attached, you must deploy your application again to apply the changes using the
`fly deploy` command.

> Read more here https://fly.io/docs/postgres/managing/attach-detach/

### Proxyng the PostgreSQL Instance Connection

Your database lives in the Fly's private network, this means that you need to gain access to this
network (using Proxies or VPNs) to connect to the database.

Fly CLI provides a `proxy` you can use for this purpose, you can use the following command:

```bash
fly proxy 5432:5432 -a <POSTGRES_APP_NAME>
```

While the proxy is running, you will be able to use your localhost at port `5432` to connect
to the database using the same connection string provided by the Fly CLI.

## Configuring DNS to use a Custom Domain

Usually I buy and manage my domains using Cloudflare, Cloudflare makes a great job managing
DNS servers.

In the following section we will use a custom domain to expose our Rust application hosted in
Fly.io to the world.

First we need to create a certificate for our domain, visit the Fly.io certificate management
page for your app at:

```bash
https://fly.io/apps/<APP_NAME>/certificates
```

> In my case this would be `https://fly.io/apps/my-rust-app/certificates`.

Click on the `Add Certificate` button and fill the domain prompt with your desired domain.
Then you will be prompted for DNS configuration.

### Domain ownership verification

You will need to visit your DNS provider and fill up a `CNAME` record with the value
provided in Fly.io's UI.

> If you are using Cloudflare, make sure you set the `DNS Only` option instead of `Proxied`.

### DNS Records for `A` and `AAAA`

You will be provided with two IP addresses, one for `A` and one for `AAAA`, you will need to
create two `A` records in your DNS provider with these values.


Once configured you can verify certificate creation by running the following command:

```bash
fly -a <APP_NAME> certs show <DOMAIN>
```

You should see something like:

```
Hostname                  = my-rust-app.fly.dev
DNS Provider              = cloudflare
Certificate Authority     = Let's Encrypt
Issued                    = rsa,ecdsa
Added to App              = X minutes ago
Source                    = fly
```

### Redirecting HTTP requests to HTTPS in Cloudflare

If you are using Cloudflare, you will notice that HTTP requests are not redirected to HTTPS.

This can be fixed by using a `Rule`, `Rule`s allows you to provide mappings to URL patterns
and HTTP responses.

Visit the `Rules` section inside your domain page in Cloudflare, then create a new rule from
the template "Redirect from HTTP to HTTPS".

Finally click on **Deploy**.

## Conclusion

It doesn't have to be only Rust, any containerized application can be deployed to Fly.io,
as you can see its easy to generate Let's Encrypt based SSL Certificates and also to
setup DNS records for your custom domain.

Other options for data persistance are also available, but remember that PostgreSQL
Solution offered by Fly.io is also another Fly.io App, so you could basically setup
something similar for a MariaDB or MongoDB if you wanted.
