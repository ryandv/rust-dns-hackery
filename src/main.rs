use core::future::Future;
use core::task::Context;
use futures::channel::mpsc;
use std::boxed::Box;
use std::pin::Pin;
use std::task::Poll;
use tokio::net::TcpStream;
use trust_dns_client::client::{ AsyncClient };
use trust_dns_proto::xfer::StreamHandle;
use trust_dns_proto::iocompat::AsyncIo02As03;
use trust_dns_proto::TokioTime;
use trust_dns_proto::tcp::Connect;
use trust_dns_client::tcp::{ TcpClientStream };
use trust_dns_server::server::{ Request, RequestHandler, ResponseHandler, ServerFuture };

struct DNSRequestHandler {
}

struct DNSResponse {
}

impl DNSResponse {
    pub fn new() -> DNSResponse {
        DNSResponse { }
    }
}

impl Future for DNSResponse {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let upstream_ns = "172.16.42.1:53".parse().unwrap();
        let (stream, handle) = TcpClientStream::<AsyncIo02As03<TcpStream>>::new::<TokioTime>(upstream_ns);
        let (send, recv) = mpsc::unbounded();
        let dns_handle = Box::new(StreamHandle::new(send));
        let async_client = AsyncClient::new(
            stream,
            handle,
            None
        );
        Poll::Ready(())
    }
}

impl DNSRequestHandler {
    pub fn new() -> DNSRequestHandler {
        DNSRequestHandler {}
    }
}

impl RequestHandler for DNSRequestHandler {
    type ResponseFuture = DNSResponse;
    fn handle_request<H: ResponseHandler>(
        &self,
        request: Request,
        response_handle: H
    ) -> Self::ResponseFuture {
        DNSResponse::new()
    }
}

fn main() {
    let handler = DNSRequestHandler::new();
    let dns_server = ServerFuture::new(handler);
}
