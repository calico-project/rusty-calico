//!
//! RPC adaptor struct use by the Wallet framework.
//!

use std::sync::Arc;

pub use calico_rpc_core::api::ctl::RpcCtl;
pub use calico_rpc_core::api::rpc::RpcApi;
pub use calico_rpc_core::notify::mode::NotificationMode;
pub use calico_wrpc_client::client::{ConnectOptions, ConnectStrategy};
pub use calico_wrpc_client::Resolver;
pub use calico_wrpc_client::WrpcEncoding;

/// Type alias for [`dyn RpcApi`](RpcApi).
pub type DynRpcApi = dyn RpcApi;
/// Type alias for a concrete [`Channel`](calico_utils::channel::Channel)
/// used for handling RPC [`Notification`](calico_rpc_core::Notification) events.
pub type NotificationChannel = calico_utils::channel::Channel<calico_rpc_core::Notification>;

/// RPC adaptor class that holds the [`RpcApi`]
/// and [`RpcCtl`] instances.
#[derive(Clone)]
pub struct Rpc {
    pub rpc_api: Arc<DynRpcApi>,
    pub rpc_ctl: RpcCtl,
}

impl Rpc {
    pub fn new(rpc_api: Arc<DynRpcApi>, rpc_ctl: RpcCtl) -> Self {
        Rpc { rpc_api, rpc_ctl }
    }

    pub fn rpc_api(&self) -> &Arc<DynRpcApi> {
        &self.rpc_api
    }

    pub fn rpc_ctl(&self) -> &RpcCtl {
        &self.rpc_ctl
    }
}
