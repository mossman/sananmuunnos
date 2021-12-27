# Sananmuunnos

This thing does exhaustive search of "Spoonerisms" in Finnish.

## Things

* `sananmuunnos` - cli version
* `sananmuunnos-webapp` - rocket application to provide web interface

## Compilation

This is a standard rust project, so cargo works as it should.

## Javascript

Javascript app is in `sananmuunnos-js`. It uses vite, svelte and tailwind css.
`npm run dev` provides dev server.

## Environment

`sananmuunnos-webapp` requires two environment variables to run.

* `STATIC_DIR` should be location of static content. Locally `sananmuunnos-js/dist` works.
* `ROCKET_DATABASE` should be database config, in form `'{sananmuunnos_db={url="postgres://<dbuser>:<dbpassword>@<dbhost>/<dbname>"}`

## Migrations

* Use `diesel migration run` to create tables on target postgres.
