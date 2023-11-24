fn main() {
    let chat_service = anemo_build::manual::Service::builder()
        .name("ChatPeer")
        .package("chat")
        .method(
            anemo_build::manual::Method::builder()
                .name("send_message")
                .route_name("SendMessage")
                .request_type("crate::chat::ChatRequest")
                .response_type("crate::chat::ChatResponse")
                .codec_path("anemo::rpc::codec::BincodeCodec")
                .build(),
        )
        .build();

    let discovery_service = anemo_build::manual::Service::builder()
        .name("Discovery")
        .package("chat")
        .method(
            anemo_build::manual::Method::builder()
                .name("get_known_peers")
                .route_name("GetKnownPeers")
                .request_type("()")
                .response_type("crate::discovery::GetKnownPeersResponse")
                .codec_path("anemo::rpc::codec::BincodeCodec")
                .build(),
        )
        .build();

    anemo_build::manual::Builder::new().compile(&[chat_service, discovery_service]);
}
