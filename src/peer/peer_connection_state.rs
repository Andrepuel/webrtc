use std::fmt;

/// PeerConnectionState indicates the state of the PeerConnection.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PeerConnectionState {
    Unspecified,

    /// PeerConnectionStateNew indicates that any of the ICETransports or
    /// DTLSTransports are in the "new" state and none of the transports are
    /// in the "connecting", "checking", "failed" or "disconnected" state, or
    /// all transports are in the "closed" state, or there are no transports.
    New,

    /// PeerConnectionStateConnecting indicates that any of the
    /// ICETransports or DTLSTransports are in the "connecting" or
    /// "checking" state and none of them is in the "failed" state.
    Connecting,

    /// PeerConnectionStateConnected indicates that all ICETransports and
    /// DTLSTransports are in the "connected", "completed" or "closed" state
    /// and at least one of them is in the "connected" or "completed" state.
    Connected,

    /// PeerConnectionStateDisconnected indicates that any of the
    /// ICETransports or DTLSTransports are in the "disconnected" state
    /// and none of them are in the "failed" or "connecting" or "checking" state.
    Disconnected,

    /// PeerConnectionStateFailed indicates that any of the ICETransports
    /// or DTLSTransports are in a "failed" state.
    Failed,

    /// PeerConnectionStateClosed indicates the peer connection is closed
    /// and the isClosed member variable of PeerConnection is true.
    Closed,
}

impl Default for PeerConnectionState {
    fn default() -> Self {
        PeerConnectionState::Unspecified
    }
}

const PEER_CONNECTION_STATE_NEW_STR: &str = "new";
const PEER_CONNECTION_STATE_CONNECTING_STR: &str = "connecting";
const PEER_CONNECTION_STATE_CONNECTED_STR: &str = "connected";
const PEER_CONNECTION_STATE_DISCONNECTED_STR: &str = "disconnected";
const PEER_CONNECTION_STATE_FAILED_STR: &str = "failed";
const PEER_CONNECTION_STATE_CLOSED_STR: &str = "closed";

impl From<&str> for PeerConnectionState {
    fn from(raw: &str) -> Self {
        match raw {
            PEER_CONNECTION_STATE_NEW_STR => PeerConnectionState::New,
            PEER_CONNECTION_STATE_CONNECTING_STR => PeerConnectionState::Connecting,
            PEER_CONNECTION_STATE_CONNECTED_STR => PeerConnectionState::Connected,
            PEER_CONNECTION_STATE_DISCONNECTED_STR => PeerConnectionState::Disconnected,
            PEER_CONNECTION_STATE_FAILED_STR => PeerConnectionState::Failed,
            PEER_CONNECTION_STATE_CLOSED_STR => PeerConnectionState::Closed,
            _ => PeerConnectionState::Unspecified,
        }
    }
}

impl From<u8> for PeerConnectionState {
    fn from(v: u8) -> Self {
        match v {
            1 => PeerConnectionState::New,
            2 => PeerConnectionState::Connecting,
            3 => PeerConnectionState::Connected,
            4 => PeerConnectionState::Disconnected,
            5 => PeerConnectionState::Failed,
            6 => PeerConnectionState::Closed,
            _ => PeerConnectionState::Unspecified,
        }
    }
}

impl fmt::Display for PeerConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            PeerConnectionState::New => PEER_CONNECTION_STATE_NEW_STR,
            PeerConnectionState::Connecting => PEER_CONNECTION_STATE_CONNECTING_STR,
            PeerConnectionState::Connected => PEER_CONNECTION_STATE_CONNECTED_STR,
            PeerConnectionState::Disconnected => PEER_CONNECTION_STATE_DISCONNECTED_STR,
            PeerConnectionState::Failed => PEER_CONNECTION_STATE_FAILED_STR,
            PeerConnectionState::Closed => PEER_CONNECTION_STATE_CLOSED_STR,
            PeerConnectionState::Unspecified => crate::UNSPECIFIED_STR,
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum NegotiationNeededState {
    /// NegotiationNeededStateEmpty not running and queue is empty
    Empty,
    /// NegotiationNeededStateEmpty running and queue is empty
    Run,
    /// NegotiationNeededStateEmpty running and queue
    Queue,
}

impl Default for NegotiationNeededState {
    fn default() -> Self {
        NegotiationNeededState::Empty
    }
}

impl From<u8> for NegotiationNeededState {
    fn from(v: u8) -> Self {
        match v {
            1 => NegotiationNeededState::Run,
            2 => NegotiationNeededState::Queue,
            _ => NegotiationNeededState::Empty,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_peer_connection_state() {
        let tests = vec![
            (crate::UNSPECIFIED_STR, PeerConnectionState::Unspecified),
            ("new", PeerConnectionState::New),
            ("connecting", PeerConnectionState::Connecting),
            ("connected", PeerConnectionState::Connected),
            ("disconnected", PeerConnectionState::Disconnected),
            ("failed", PeerConnectionState::Failed),
            ("closed", PeerConnectionState::Closed),
        ];

        for (state_string, expected_state) in tests {
            assert_eq!(
                expected_state,
                PeerConnectionState::from(state_string),
                "testCase: {}",
                expected_state,
            );
        }
    }

    #[test]
    fn test_peer_connection_state_string() {
        let tests = vec![
            (PeerConnectionState::Unspecified, crate::UNSPECIFIED_STR),
            (PeerConnectionState::New, "new"),
            (PeerConnectionState::Connecting, "connecting"),
            (PeerConnectionState::Connected, "connected"),
            (PeerConnectionState::Disconnected, "disconnected"),
            (PeerConnectionState::Failed, "failed"),
            (PeerConnectionState::Closed, "closed"),
        ];

        for (state, expected_string) in tests {
            assert_eq!(expected_string, state.to_string(),)
        }
    }
}
