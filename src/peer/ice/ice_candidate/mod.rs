pub mod ice_candidate_pair;
pub mod ice_candidate_type;

use crate::error::Error;
use crate::peer::ice::ice_candidate::ice_candidate_type::ICECandidateType;
use crate::peer::ice::ice_protocol::ICEProtocol;
use anyhow::Result;
use ice::candidate::candidate_base::CandidateBaseConfig;
use ice::candidate::candidate_host::CandidateHostConfig;
use ice::candidate::candidate_peer_reflexive::CandidatePeerReflexiveConfig;
use ice::candidate::candidate_relay::CandidateRelayConfig;
use ice::candidate::candidate_server_reflexive::CandidateServerReflexiveConfig;
use ice::candidate::Candidate;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

/// ICECandidate represents a ice candidate
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ICECandidate {
    pub stats_id: String,
    pub foundation: String,
    pub priority: u32,
    pub address: String,
    pub protocol: ICEProtocol,
    pub port: u16,
    pub typ: ICECandidateType,
    pub component: u16,
    pub related_address: String,
    pub related_port: u16,
    pub tcp_type: String,
}

/// Conversion for package ice
pub(crate) fn ice_candidates_from_ice(
    ice_candidates: &[Arc<dyn Candidate + Send + Sync>],
) -> Vec<ICECandidate> {
    ice_candidates.iter().map(|c| c.into()).collect()
}

impl From<&Arc<dyn Candidate + Send + Sync>> for ICECandidate {
    fn from(c: &Arc<dyn Candidate + Send + Sync>) -> Self {
        let typ: ICECandidateType = c.candidate_type().into();
        let protocol = ICEProtocol::from(c.network_type().network_short().as_str());
        let (related_address, related_port) = if let Some(ra) = c.related_address() {
            (ra.address, ra.port)
        } else {
            (String::new(), 0)
        };

        ICECandidate {
            stats_id: c.id(),
            foundation: c.foundation(),
            priority: c.priority(),
            address: c.address(),
            protocol,
            port: c.port(),
            component: c.component(),
            typ,
            tcp_type: c.tcp_type().to_string(),
            related_address,
            related_port,
        }
    }
}

impl ICECandidate {
    pub(crate) async fn to_ice(&self) -> Result<impl Candidate> {
        let candidate_id = self.stats_id.clone();
        let c = match self.typ {
            ICECandidateType::Host => {
                let config = CandidateHostConfig {
                    base_config: CandidateBaseConfig {
                        candidate_id,
                        network: self.protocol.to_string(),
                        address: self.address.clone(),
                        port: self.port,
                        component: self.component,
                        //tcp_type: ice.NewTCPType(c.TCPType),
                        foundation: self.foundation.clone(),
                        priority: self.priority,
                        ..Default::default()
                    },
                    ..Default::default()
                };
                config.new_candidate_host().await?
            }
            ICECandidateType::Srflx => {
                let config = CandidateServerReflexiveConfig {
                    base_config: CandidateBaseConfig {
                        candidate_id,
                        network: self.protocol.to_string(),
                        address: self.address.clone(),
                        port: self.port,
                        component: self.component,
                        foundation: self.foundation.clone(),
                        priority: self.priority,
                        ..Default::default()
                    },
                    rel_addr: self.related_address.clone(),
                    rel_port: self.related_port,
                };
                config.new_candidate_server_reflexive().await?
            }
            ICECandidateType::Prflx => {
                let config = CandidatePeerReflexiveConfig {
                    base_config: CandidateBaseConfig {
                        candidate_id,
                        network: self.protocol.to_string(),
                        address: self.address.clone(),
                        port: self.port,
                        component: self.component,
                        foundation: self.foundation.clone(),
                        priority: self.priority,
                        ..Default::default()
                    },
                    rel_addr: self.related_address.clone(),
                    rel_port: self.related_port,
                };
                config.new_candidate_peer_reflexive().await?
            }
            ICECandidateType::Relay => {
                let config = CandidateRelayConfig {
                    base_config: CandidateBaseConfig {
                        candidate_id,
                        network: self.protocol.to_string(),
                        address: self.address.clone(),
                        port: self.port,
                        component: self.component,
                        foundation: self.foundation.clone(),
                        priority: self.priority,
                        ..Default::default()
                    },
                    rel_addr: self.related_address.clone(),
                    rel_port: self.related_port,
                    relay_client: None, //TODO?
                };
                config.new_candidate_relay().await?
            }
            _ => return Err(Error::ErrICECandidateTypeUnknown.into()),
        };

        Ok(c)
    }

    /// to_json returns an ICECandidateInit
    /// as indicated by the spec https://w3c.github.io/webrtc-pc/#dom-rtcicecandidate-tojson
    pub async fn to_json(&self) -> Result<ICECandidateInit> {
        let candidate = self.to_ice().await?;

        Ok(ICECandidateInit {
            candidate: format!("candidate:{}", candidate.marshal()),
            sdp_mid: "".to_owned(),
            sdp_mline_index: 0u16,
            username_fragment: "".to_owned(),
        })
    }
}

impl fmt::Display for ICECandidate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}:{}{}",
            self.protocol, self.typ, self.address, self.port, self.related_address,
        )
    }
}

/// ICECandidateInit is used to serialize ice candidates
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ICECandidateInit {
    pub candidate: String,
    pub sdp_mid: String,
    pub sdp_mline_index: u16,
    pub username_fragment: String,
}
