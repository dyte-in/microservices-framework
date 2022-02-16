pub mod broker;
pub mod context;
pub mod logger;
pub mod packet;
pub mod registry;
pub mod service;
pub mod strategies;
pub use broker::HandlerResult;
pub use broker::ServiceBroker;
pub use broker::ServiceBrokerMessage;
pub use registry::Registry;
pub use service::Service;

const INTERNAL_PREFIX: char = '$';
