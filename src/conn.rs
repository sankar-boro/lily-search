use scylla::batch::Batch;
use scylla::{
    Session, 
    SessionBuilder, 
    transport::errors::NewSessionError
};

use scylla::{QueryResult, BatchResult};
use scylla::query::Query;
use scylla::frame::value::ValueList;
use scylla::frame::value::BatchValues;
use scylla::transport::errors::QueryError;

use log::{error};

pub async fn get_session() -> Session {
    let uri = "127.0.0.1:9042";
    let session = SessionBuilder::new().known_node(uri).build().await;
    if let Err(err) = session {
        match err {
            NewSessionError::FailedToResolveAddress(e) => error!("FailedToResolveAddress, {}", e),
            NewSessionError::EmptyKnownNodesList => error!("EmptyKnownNodesList"),
            NewSessionError::DbError(e, er) => error!("DbError, {} {}", e, er),
            NewSessionError::BadQuery(e) => error!("BadQuery, {}", e),
            NewSessionError::IoError(e) => {
                error!("IoError, {}", e);
                println!("Would you mind to check if you have started scylladb service. Command is: \"sudo systemctl start scylla-server\" ");
            },
            NewSessionError::ProtocolError(e) => error!("ProtocolError, {}", e),
            NewSessionError::InvalidMessage(e) => error!("InvalidMessage, {}", e),
            NewSessionError::TimeoutError => error!("TimeoutError"),
        }
        panic!("Could not start server");
    }
    println!("wtd");
    session.unwrap()
}