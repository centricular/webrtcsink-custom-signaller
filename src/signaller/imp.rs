use gst::glib;
use gst::subclass::prelude::*;
use webrtcsink::webrtcsink::WebRTCSink;

#[derive(Default)]
pub struct Signaller {}

impl Signaller {
    pub fn start(&self, _element: &WebRTCSink) {
        unimplemented!()
    }

    pub fn handle_sdp(
        &self,
        _element: &WebRTCSink,
        _peer_id: &str,
        _sdp: &gst_webrtc::WebRTCSessionDescription,
    ) {
        unimplemented!()
    }

    pub fn handle_ice(
        &self,
        _element: &WebRTCSink,
        _peer_id: &str,
        _candidate: &str,
        _sdp_mline_index: Option<u32>,
        _sdp_mid: Option<String>,
    ) {
        unimplemented!()
    }

    pub fn stop(&self, _element: &WebRTCSink) {
        unimplemented!()
    }

    pub fn consumer_removed(&self, _element: &WebRTCSink, _peer_id: &str) {
        unimplemented!()
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Signaller {
    const NAME: &'static str = "MyCustomRTCSinkSignaller";
    type Type = super::Signaller;
    type ParentType = glib::Object;
}

impl ObjectImpl for Signaller {}
