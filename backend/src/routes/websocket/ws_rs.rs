use rocket::serde::{self};
use rocket::tokio::runtime::Runtime;
use rocket::tokio::task::LocalSet;
use sqlx::postgres::PgArguments;
use sqlx::query::Query;
use sqlx::{Postgres, PgPool};
use ws::{
    listen, CloseCode, Error, Handler, Handshake, Message, Request, Response, Result, Sender,
};

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::database;
use crate::routes::websocket::models::WSMessage;

// Server web application handler
struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
    connections: Arc<Mutex<HashMap<String, Sender>>>,
    db: PgPool,
}

impl Handler for Server {
    // used once for const socket = new WebSocket("wss://" + window.location.host + "/ws");
    // and no need for reconnect later
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        //Removed enclosing "()" from "Response"

        match req.resource() {
            _ if req.resource().contains("/api/ws") => {
                println!(
                    "Browser Request from {:?}",
                    req.origin().unwrap_or(Some("")).unwrap_or("")
                );
                println!("Client found is {:?}", req.client_addr().unwrap());
                let resp = Response::from_request(req);
                // println!("{:?} \n", &resp);
                resp
            }

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    /// Handles the registration of the connections to the hashmap to then send only to those
    /// the message is destined to
    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        self.count.set(self.count.get() + 1);

        if let Some(param) = handshake.request.resource().split("?").last() {
            if let Some(user_id) = param.split("=").last() {
                self.connections
                    .lock()
                    .unwrap()
                    .insert(user_id.to_string(), self.out.clone());
                Ok(())
            } else {
                println!("No user_value in param");
                Ok(())
            }
        } else {
            println!("No user_id param");
            Ok(())
        }
    }

    /// Handle messages recieved in the websocket and process them
    fn on_message(&mut self, message: Message) -> Result<()> {
        let raw_message = message.into_text()?;
        println!("The message from the client is {:#?}", &raw_message);

        let parsed_message = match serde::json::from_str::<WSMessage>(&raw_message) {
            Ok(msg) => msg,
            Err(e) => {
                println!("Error parsing WebSocket message: {:?}", e);
                return Ok(());
            }
        };
        let connections = self.connections.lock().unwrap();
        dbg!(&connections);

        match parsed_message.r#type.as_str() {
            "broadcast" => self.out.broadcast(Message::Text(raw_message)),
            "chat" => {
                // Used to perform asynchronous blocks in a synchronous code with tokio.
                let mut rt: Runtime = Runtime::new().unwrap();
                let local: LocalSet = LocalSet::new();

                local.block_on(&mut rt, async {
                    let query: Query<'_, Postgres, PgArguments> =
                        sqlx::query("INSERT INTO CHAT(`from`, `to`, `content`, `timestamp`) VALUES(?, ?, ?, NOW())")
                            .bind(&parsed_message.from)
                            .bind(&parsed_message.to)
                            .bind(&parsed_message.data);
                    match query.execute(&self.db).await {
                        Ok(_) => {
                            println!("Inserted last chat!")
                        }
                        Err(e) => println!("{e}"),
                    };
                });
                if let (Some(sender), Some(receiver)) = (
                    connections.get(&parsed_message.from),
                    connections.get(&parsed_message.to),
                ) {
                    let _ = sender.send(Message::Text(raw_message.clone()));
                    receiver.send(Message::Text(raw_message.clone()))
                } else if let (Some(sender), None) = (
                    connections.get(&parsed_message.from),
                    connections.get(&parsed_message.to),
                ) {
                    println!("Receiver not online");
                    sender.send(Message::Text(raw_message.clone()))
                } else {
                    println!("Nope");
                    Ok(())
                }
            }
            "keep-alive" => {
                if let Some(receiver) = connections.get(&parsed_message.from) {
                    receiver.send(Message::Text(raw_message.clone()))
                } else {
                    Ok(())
                }
            }
            _ => self
                .out
                .broadcast(Message::Text(String::from("Type unknown"))),
        }

        // Broadcast to all connections
        // self.out.broadcast(message)
    }

    fn on_close(&mut self, _code: CloseCode, _reason: &str) {
        //TODO REFACTOR maybe add a token for websocket in database and delete it when it disconnects to remove it from hashmap
        // self.out.token()
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

/// Function to start the websocket server and initializing
/// the `HashMap` of websockets connections alive
pub async fn websocket() -> () {
    println!("Web Socket Server is ready at ws://0.0.0.0:7777/ws");
    println!("Server is ready at http://0.0.0.0:7777/");

    // Rc is a reference-counted box for sharing the count between handlers
    // since each handler needs to own its contents.
    // Cell gives us interior mutability so we can increment
    // or decrement the count between handlers.

    // Listen on an address and call the closure for each connection
    let count = Rc::new(Cell::new(0));
    let connections = Arc::new(Mutex::new(HashMap::new()));
    let db: PgPool = database::open()
        .await
        .unwrap_or_else(|e| panic!("Couldn't open database: {e}"));
    listen("0.0.0.0:7777", |out| Server {
        out,
        count: count.clone(),
        connections: connections.clone(),
        db: db.clone(),
    })
    .unwrap()
}
