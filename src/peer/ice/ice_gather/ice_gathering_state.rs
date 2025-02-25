use std::fmt;

/// ICEGatheringState describes the state of the candidate gathering process.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ICEGatheringState {
    Unspecified,

    /// ICEGatheringStateNew indicates that any of the ICETransports are
    /// in the "new" gathering state and none of the transports are in the
    /// "gathering" state, or there are no transports.
    New,

    /// ICEGatheringStateGathering indicates that any of the ICETransports
    /// are in the "gathering" state.
    Gathering,

    /// ICEGatheringStateComplete indicates that at least one ICETransport
    /// exists, and all ICETransports are in the "completed" gathering state.
    Complete,
}

impl Default for ICEGatheringState {
    fn default() -> Self {
        ICEGatheringState::Unspecified
    }
}

const ICE_GATHERING_STATE_NEW_STR: &str = "new";
const ICE_GATHERING_STATE_GATHERING_STR: &str = "gathering";
const ICE_GATHERING_STATE_COMPLETE_STR: &str = "complete";

/// takes a string and converts it to ICEGatheringState
impl From<&str> for ICEGatheringState {
    fn from(raw: &str) -> Self {
        match raw {
            ICE_GATHERING_STATE_NEW_STR => ICEGatheringState::New,
            ICE_GATHERING_STATE_GATHERING_STR => ICEGatheringState::Gathering,
            ICE_GATHERING_STATE_COMPLETE_STR => ICEGatheringState::Complete,
            _ => ICEGatheringState::Unspecified,
        }
    }
}

impl fmt::Display for ICEGatheringState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ICEGatheringState::New => write!(f, "{}", ICE_GATHERING_STATE_NEW_STR),
            ICEGatheringState::Gathering => write!(f, "{}", ICE_GATHERING_STATE_GATHERING_STR),
            ICEGatheringState::Complete => {
                write!(f, "{}", ICE_GATHERING_STATE_COMPLETE_STR)
            }
            _ => write!(f, "{}", crate::UNSPECIFIED_STR),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_ice_gathering_state() {
        let tests = vec![
            ("Unspecified", ICEGatheringState::Unspecified),
            ("new", ICEGatheringState::New),
            ("gathering", ICEGatheringState::Gathering),
            ("complete", ICEGatheringState::Complete),
        ];

        for (state_string, expected_state) in tests {
            assert_eq!(expected_state, ICEGatheringState::from(state_string));
        }
    }

    #[test]
    fn test_ice_gathering_state_string() {
        let tests = vec![
            (ICEGatheringState::Unspecified, "Unspecified"),
            (ICEGatheringState::New, "new"),
            (ICEGatheringState::Gathering, "gathering"),
            (ICEGatheringState::Complete, "complete"),
        ];

        for (state, expected_string) in tests {
            assert_eq!(expected_string, state.to_string());
        }
    }
}
