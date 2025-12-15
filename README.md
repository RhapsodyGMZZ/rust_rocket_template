> [!CAUTION]
> First of all, type this in your console:

`git rm --cached .env`

# How to run it ?
Just type this command : <br>
```docker compose up -d``` to start the database <br>
and then type this: <br>
```cargo install cargo-watch --locked && cargo watch -x "run"```

## Why did I created this template ?

Because I took so long when I was doing my very first project with Rocket. Sooo... I decided that nobody has to take any longer with this template.

## Features 

### File server 
There is a file server already implemented, binded on a ```/public``` path on the application. Feel free to rename it if you want to. It points to the ```/static``` in the backend root directory.

### Templating and context with ***.tera*** files
Rocket offers differents templating models, I decided to use the Tera one (because i like this templating model). Go read [the official documentation](https://api.rocket.rs/master/rocket_dyn_templates/tera/) of Rust Rocket about this model.

### Dockerization of database
I wanted to be the most accessible as possible. What's better than docker to wrap a database with a persistent volume ? The database is created with all the ```/backend/.env.database``` data.

### CORS policy

I added the CORS header as a [Fairing](https://api.rocket.rs/master/rocket/fairing/) to avoid errrors when developping your project.

### Database MYSQL connection

The default driver is a Postgres driver. Feel free to use MYSQL's one or another.

### Migrations as 'code-first' execution

At the restart of the server, the migrations unapplied are applied.

### Added Handling of websockets (with a fake "chat" message handler to show how to do a SQL request when receiving a WS message)
It run simultaneously on 7777 port. This port accepts connection to 'upgrade it' as a  websocket connection.

RhapsodyGMZZ&copy;
