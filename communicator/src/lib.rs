pub mod network {
    pub fn connect() {
    }

    pub mod client {
        pub fn connect() {
        }
    }
}

pub mod client {
	pub fn connect() {
	}
}

#[cfg(test)]
mod tests {
	use super::client;
 
    #[test]
    fn it_works() {
        client::connect();
    }
}
