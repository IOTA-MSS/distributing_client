

// #[derive(Debug)]
// pub struct DistributionState {
//     pub contract: TangleTunesClient,
//     /// The folder from which music is served
//     pub db: Database,
//     /// The port on which to serve
//     pub port: u16,
// }

// impl DistributionState {
//     /// Initializes the global application state
//     pub async fn new(
//         db_path: &str,
//         port: u16,
//         key: &str,
//         node_url: &str,
//         contract_address: &str,
//     ) -> eyre::Result<Self> {


//         Ok(Self { db, port, contract })
//     }

//     pub fn leak(self) -> &'static Self {
//         Box::leak(Box::new(self))
//     }
// }
