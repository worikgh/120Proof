//! Manipulate the pipes that drive the audio through the pedals.
use jack::Error;

pub struct JackConnections {
    client: jack::Client,
    last_connection: Option<(String, String)>,
}

impl JackConnections {
    pub fn make_connection(&mut self, src: String, dst: String) -> Result<(), Error> {
        // Connect the new connection, then delete the old one, then
        // store the new connection

        self.client
            .connect_ports_by_name(src.as_str(), dst.as_str())?;
        if let Some((src, dst)) = &self.last_connection {
            self.client
                .disconnect_ports_by_name(src.as_str(), dst.as_str())?;
        }
        eprintln!("End: make_connection({}, {})", &src, &dst);
        self.last_connection = Some((src, dst));

        Ok(())
    }

    pub fn new(client_name: &str) -> Self {
        JackConnections {
            client: jack::Client::new(client_name, jack::ClientOptions::NO_START_SERVER)
                .unwrap()
                .0,
            last_connection: None,
        }
    }
}
