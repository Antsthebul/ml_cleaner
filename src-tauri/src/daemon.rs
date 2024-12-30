// Background task runner

use std::net::Ipv4Addr;

// use ml_cleaner::file_config::Configuration;


pub struct ModelHubRecord {
    // pub machine_ip: Ipv4Addr,
    // pub project_name: String,
    pub provider: String,
    pub machine_id: String,
}

// Returns a list of Records related to machines
// pub fn gather_existing_machines_in_config(config: &Configuration) -> Vec<ModelHubRecord> {
//     let mut records = Vec::new();
//     for p in &config.projects {
//         let project_name = &p.name;
//         for d in &p.deployments {
//             for m in &d.machines {
//                 if let Some(ip) = m.ip_addr {
//                     records.push(ModelHubRecord {
//                         machine_ip: ip,
//                         project_name: project_name.to_owned(),
//                         provider: m.provider.to_owned(),
//                         machine_id: m.id.to_owned(),
//                     })
//                 }
//             }
//         }
//     }

//     records
// }
