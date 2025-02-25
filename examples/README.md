<h1 align="center">
  Examples
</h1>

All examples are ported from [Pion](https://github.com/pion/webrtc/tree/master/examples#readme). Please check [Pion Examples](https://github.com/pion/webrtc/tree/master/examples#readme) for more details:

#### Media API
- [ ] [Reflect](reflect): The reflect example demonstrates how to have webrtc-rs send back to the user exactly what it receives using the same PeerConnection.
- [ ] [Play from Disk](play-from-disk): The play-from-disk example demonstrates how to send video to your browser from a file saved to disk.
- [ ] [Play from Disk Renegotation](play-from-disk-renegotation): The play-from-disk-renegotation example is an extension of the play-from-disk example, but demonstrates how you can add/remove video tracks from an already negotiated PeerConnection.
- [ ] [Insertable Streams](insertable-streams): The insertable-streams example demonstrates how webrtc-rs can be used to send E2E encrypted video and decrypt via insertable streams in the browser.
- [ ] [Save to Disk](save-to-disk): The save-to-disk example shows how to record your webcam and save the footage to disk on the server side.
- [ ] [Broadcast](broadcast): The broadcast example demonstrates how to broadcast a video to multiple peers. A broadcaster uploads the video once and the server forwards it to all other peers.
- [ ] [RTP Forwarder](rtp-forwarder): The rtp-forwarder example demonstrates how to forward your audio/video streams using RTP.
- [ ] [RTP to WebRTC](rtp-to-webrtc): The rtp-to-webrtc example demonstrates how to take RTP packets sent to a webrtc-rs process into your browser.
- [ ] [Simulcast](simulcast): The simulcast example demonstrates how to accept and demux 1 Track that contains 3 Simulcast streams. It then returns the media as 3 independent Tracks back to the sender.

#### Data Channel API
- [ ] [Data Channels](data-channels): The data-channels example shows how you can send/recv DataChannel messages from a web browser.
- [ ] [Data Channels Create](data-channels-create): Example data-channels-create shows how you can send/recv DataChannel messages from a web browser. The difference with the data-channels example is that the data channel is initialized from the server side in this example.
- [ ] [Data Channels Close](data-channels-close): Example data-channels-close is a variant of data-channels that allow playing with the life cycle of data channels.
- [ ] [Data Channels Detach](data-channels-detach): The data-channels-detach example shows how you can send/recv DataChannel messages using the underlying DataChannel implementation directly. This provides a more idiomatic way of interacting with Data Channels.
- [ ] [Data Channels Detach Create](data-channels-detach-create): Example data-channels-detach-create shows how you can send/recv DataChannel messages using the underlying DataChannel implementation directly. This provides a more idiomatic way of interacting with Data Channels. The difference with the data-channels-detach example is that the data channel is initialized in this example.
- [ ] [ORTC](ortc): Example ortc shows how you an use the ORTC API for DataChannel communication.
- [ ] [Offer Answer](offer-answer): Example offer-answer is an example of two webrtc-rs or pion instances communicating directly!
