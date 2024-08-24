/// Components is the core of the library crate. This is all the 
/// funcdemental logic for what 'components' to use and should be 
/// exapnded upon. For example, a repository (default: file) 
/// a ML workspace (default:paperspace), and a ML data store(AWS) 
/// will make up the usable commponents. For web APIS, CLIs, or desktop
/// app, which is controlled by the binary crate.
pub mod adapters;
pub mod repository;

