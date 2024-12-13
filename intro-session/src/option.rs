// Oversimplified sdk code
// - No error handling
// - No async
// .. you get the point
#[allow(dead_code)]
pub mod sdk {
    pub struct Cluster {
        pub name: String,
        pub replicaset_config: ReplicasetConfig,
        //pub replicaset_config: Option<ReplicasetConfig>,
    }

    pub struct ReplicasetConfig {
        pub number_of_hosts: u32,
    }

    pub fn get_cluster(_id: u128) -> Cluster {
        Cluster {
            name: "My fantastic cluster".into(),
            replicaset_config: ReplicasetConfig { number_of_hosts: 3 },
            //replicaset_config: None,
        }
    }
}

pub fn sdk_consumer() {
    // get a cluster using the SDK
    let cluster = sdk::get_cluster(123);

    // if the number of hosts in the replicaset is higher than 1 print, nice!
    if cluster.replicaset_config.number_of_hosts > 1 {
        println!("nice!")
    }
}
