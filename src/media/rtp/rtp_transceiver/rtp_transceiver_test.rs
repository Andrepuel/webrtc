use super::*;
use crate::api::media_engine::{MIME_TYPE_OPUS, MIME_TYPE_VP8, MIME_TYPE_VP9};
use crate::api::APIBuilder;
use crate::peer::configuration::Configuration;
use crate::peer::peer_connection::peer_connection_test::close_pair_now;

#[tokio::test]
async fn test_rtp_transceiver_set_codec_preferences() -> Result<()> {
    let mut m = MediaEngine::default();
    m.register_default_codecs()?;
    m.push_codecs(m.video_codecs.clone(), RTPCodecType::Video)
        .await;
    m.push_codecs(m.audio_codecs.clone(), RTPCodecType::Audio)
        .await;

    let media_video_codecs = m.video_codecs.clone();

    let api = APIBuilder::new().with_media_engine(m).build();

    let tr = RTPTransceiver::new(
        None,
        None,
        RTPTransceiverDirection::Unspecified,
        RTPCodecType::Video,
        media_video_codecs.clone(),
        Arc::clone(&api.media_engine),
    )
    .await;

    assert_eq!(&media_video_codecs, &tr.get_codecs().await);

    let fail_test_cases = vec![
        vec![RTPCodecParameters {
            capability: RTPCodecCapability {
                mime_type: MIME_TYPE_OPUS.to_string(),
                clock_rate: 48000,
                channels: 2,
                sdp_fmtp_line: "minptime=10;useinbandfec=1".to_string(),
                rtcp_feedback: vec![],
            },
            payload_type: 111,
            ..Default::default()
        }],
        vec![
            RTPCodecParameters {
                capability: RTPCodecCapability {
                    mime_type: MIME_TYPE_VP8.to_string(),
                    clock_rate: 90000,
                    channels: 0,
                    sdp_fmtp_line: "".to_string(),
                    rtcp_feedback: vec![],
                },
                payload_type: 96,
                ..Default::default()
            },
            RTPCodecParameters {
                capability: RTPCodecCapability {
                    mime_type: MIME_TYPE_OPUS.to_string(),
                    clock_rate: 48000,
                    channels: 2,
                    sdp_fmtp_line: "minptime=10;useinbandfec=1".to_string(),
                    rtcp_feedback: vec![],
                },
                payload_type: 111,
                ..Default::default()
            },
        ],
    ];

    for test_case in fail_test_cases {
        if let Err(err) = tr.set_codec_preferences(test_case).await {
            assert!(Error::ErrRTPTransceiverCodecUnsupported.equal(&err));
        } else {
            assert!(false);
        }
    }

    let success_test_cases = vec![
        vec![RTPCodecParameters {
            capability: RTPCodecCapability {
                mime_type: MIME_TYPE_VP8.to_string(),
                clock_rate: 90000,
                channels: 0,
                sdp_fmtp_line: "".to_string(),
                rtcp_feedback: vec![],
            },
            payload_type: 96,
            ..Default::default()
        }],
        vec![
            RTPCodecParameters {
                capability: RTPCodecCapability {
                    mime_type: MIME_TYPE_VP8.to_string(),
                    clock_rate: 90000,
                    channels: 0,
                    sdp_fmtp_line: "".to_string(),
                    rtcp_feedback: vec![],
                },
                payload_type: 96,
                ..Default::default()
            },
            RTPCodecParameters {
                capability: RTPCodecCapability {
                    mime_type: "video/rtx".to_string(),
                    clock_rate: 90000,
                    channels: 0,
                    sdp_fmtp_line: "apt=96".to_string(),
                    rtcp_feedback: vec![],
                },
                payload_type: 97,
                ..Default::default()
            },
            RTPCodecParameters {
                capability: RTPCodecCapability {
                    mime_type: MIME_TYPE_VP9.to_string(),
                    clock_rate: 90000,
                    channels: 0,
                    sdp_fmtp_line: "profile-id=0".to_string(),
                    rtcp_feedback: vec![],
                },
                payload_type: 98,
                ..Default::default()
            },
            RTPCodecParameters {
                capability: RTPCodecCapability {
                    mime_type: "video/rtx".to_string(),
                    clock_rate: 90000,
                    channels: 0,
                    sdp_fmtp_line: "apt=98".to_string(),
                    rtcp_feedback: vec![],
                },
                payload_type: 99,
                ..Default::default()
            },
        ],
    ];

    for test_case in success_test_cases {
        tr.set_codec_preferences(test_case).await?;
    }

    tr.set_codec_preferences(vec![]).await?;
    assert_ne!(0, tr.get_codecs().await.len());

    Ok(())
}

// Assert that SetCodecPreferences properly filters codecs and PayloadTypes are respected
#[tokio::test]
async fn test_rtp_transceiver_set_codec_preferences_payload_type() -> Result<()> {
    let test_codec = RTPCodecParameters {
        capability: RTPCodecCapability {
            mime_type: "video/test_codec".to_string(),
            clock_rate: 90000,
            channels: 0,
            sdp_fmtp_line: "".to_string(),
            rtcp_feedback: vec![],
        },
        payload_type: 50,
        ..Default::default()
    };

    let mut m = MediaEngine::default();
    m.register_default_codecs()?;
    let api = APIBuilder::new().with_media_engine(m).build();
    let offer_pc = api.new_peer_connection(Configuration::default()).await?;

    let mut m = MediaEngine::default();
    m.register_default_codecs()?;
    m.register_codec(test_codec.clone(), RTPCodecType::Video)?;
    let api = APIBuilder::new().with_media_engine(m).build();
    let answer_pc = api.new_peer_connection(Configuration::default()).await?;

    let _ = offer_pc
        .add_transceiver_from_kind(RTPCodecType::Video, &[])
        .await?;

    let answer_transceiver = answer_pc
        .add_transceiver_from_kind(RTPCodecType::Video, &[])
        .await?;

    answer_transceiver
        .set_codec_preferences(vec![
            test_codec,
            RTPCodecParameters {
                capability: RTPCodecCapability {
                    mime_type: MIME_TYPE_VP8.to_string(),
                    clock_rate: 90000,
                    channels: 0,
                    sdp_fmtp_line: "".to_string(),
                    rtcp_feedback: vec![],
                },
                payload_type: 51,
                ..Default::default()
            },
        ])
        .await?;

    let offer = offer_pc.create_offer(None).await?;

    offer_pc.set_local_description(offer.clone()).await?;
    answer_pc.set_remote_description(offer).await?;

    let answer = answer_pc.create_answer(None).await?;

    // VP8 with proper PayloadType
    assert!(answer.serde.sdp.contains("a=rtpmap:96 VP8/90000"));

    // test_codec is ignored since offerer doesn't support
    assert!(!answer.serde.sdp.contains("test_codec"));

    close_pair_now(&offer_pc, &answer_pc).await;

    Ok(())
}
