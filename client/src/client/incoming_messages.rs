use iced_native::{futures::stream::BoxStream, subscription::Recipe};
use protocol::network::NetworkMessage;
use std::hash::{Hasher};
use std::io::Read;

pub struct IncomingMessages {
    pub stream: std::net::TcpStream,
}

impl<H, I> Recipe<H, I> for IncomingMessages
where
    H: Hasher,
{
    type Output = NetworkMessage;

    fn hash(&self, _state: &mut H) {
    }

    fn stream(
        self: Box<Self>,
        _input: BoxStream<I>,
    ) -> BoxStream<Self::Output> {
        let stream = self.stream;

        Box::pin(iced_native::futures::stream::unfold(stream, |mut stream| async move {
            let mut buf = [0; 1024];
            let len = stream.read(&mut buf).unwrap();

            let slice = &buf[..len];
            let msg = NetworkMessage::from_slice(slice).unwrap();

            Some((msg, stream))
        }))
    }
}
