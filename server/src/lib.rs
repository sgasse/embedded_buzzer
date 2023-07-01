use std::sync::Arc;

use common::Message;
use tokio::sync::broadcast;

pub use common::GameInfo;

pub mod net_sockets;
pub mod websocket;

pub type UiBackendRouter = Arc<UiBackendRouterInner>;

pub struct UiBackendRouterInner {
    pub frontend_tx: broadcast::Sender<Message>,
    pub frontend_rx: broadcast::Receiver<Message>,
}
