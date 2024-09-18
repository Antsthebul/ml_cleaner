// Background task runner

use std::{time, net::Ipv4Addr, clone::Clone};

use app::{file_config::Configuration, Client};

pub struct ModelHubRecord{
    pub machine_ip: Ipv4Addr,
    pub project_name: String,
    pub provider: String,
    pub machine_id: String
}

/// Returns a list of Records related to machines
pub fn gather_existing_machines_in_config(config:&Configuration)-> Vec<ModelHubRecord> {
    let mut records= Vec::new();
    for p in &config.projects{
        let project_name = &p.name;
        for d in &p.deployments{
            for m in &d.machines{
                if let Some(ip) = m.ip_addr{
                    records.push(ModelHubRecord{machine_ip:ip, 
                            project_name:project_name.to_owned(), 
                              provider:m.provider.to_owned(),
                            machine_id:m.id.to_owned()
                })
                }
            }
        }
    };

    records
}

pub async fn run_daemon(c:impl Client + Clone, machine_id:String){
    println!("daemon run");
    loop{
        let client = c.clone();
        client.get_machine_status(&machine_id).await;
        tokio::time::sleep(time::Duration::from_millis(5000)).await;
    }
}