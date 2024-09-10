//! Manipulate the pipes that drive the audio through the pedals.
use jack::Error;

pub struct JackConnections {
    client: jack::Client,
}

impl JackConnections {
    pub fn unmake_connection(&mut self, src: String, dst: String) -> Result<(), Error> {
        self.client
            .disconnect_ports_by_name(src.as_str(), dst.as_str())
    }

    pub fn make_connection(&mut self, src: String, dst: String) -> Result<(), Error> {
        // Connect the new connection, then delete the old one, then
        // store the new connection

        self.client
            .connect_ports_by_name(src.as_str(), dst.as_str())?;
        eprintln!("End: make_connection({}, {})", &src, &dst);

        Ok(())
    }

    pub fn new(client_name: &str) -> Self {
        JackConnections {
            client: jack::Client::new(client_name, jack::ClientOptions::NO_START_SERVER)
                .unwrap()
                .0,
        }
    }
}
