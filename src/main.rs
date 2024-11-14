// src/main.rs

use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use hyper::{server::conn::Http, service::service_fn};
use tokio::net::TcpListener;
use dav_server::{DavHandler, fakels::FakeLs, localfs::LocalFs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Define the address to bind the server
    let addr: SocketAddr = ([0, 0, 0, 0], 4918).into();

    // Create the filesystem handler pointing to the "data" directory
    let dav_fs = LocalFs::new("data", false, false, false);

    // Create the lock system
    let dav_ls = FakeLs::new();

    // Build the DavHandler with the filesystem and lock system
    let dav_server = DavHandler::builder()
        .filesystem(dav_fs)
        .locksystem(dav_ls)
        .build_handler();

    // Create a TCP listener
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on http://{}", addr);

    // Start accepting incoming connections
    loop {
        let (stream, _) = listener.accept().await?;
        let dav_server = dav_server.clone();

        // Spawn a new task to handle the connection
        tokio::task::spawn(async move {
            // Use hyper's Http to serve the connection
            if let Err(err) = Http::new()
                .serve_connection(
                    stream,
                    service_fn(move |req| {
                        let dav_server = dav_server.clone();
                        async move {
                            // Handle the request using the DavHandler
                            Ok::<_, Infallible>(dav_server.handle(req).await)
                        }
                    }),
                )
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

// End of src/main.rs
