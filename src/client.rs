use futures::SinkExt;
use tokio::net::TcpStream;
use std::net::TcpStream as stdTcpStream;
use tokio::stream::StreamExt;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use serde::{Deserialize, Serialize};

use bytes::Bytes;

static LOCAL_REMITS: &str = "localhost:4242";
static OK_RESP: &[u8] = &[0x62, 0x6F, 0x6B];

// async pub fn integration_tests() {
//     let framer = &mut (connect_to_remits().await);

// }

pub fn new_log_add_req(name: &str) -> Vec<u8> {
    #[derive(Serialize)]
    struct Body {
        log_name: String,
    }

    let mut body = vec![0x00, 0x01];
    let req = serde_cbor::to_vec(&Body {
        log_name: name.into(),
    })
    .unwrap();
    body.extend(req);
    body
}

pub fn new_log_show_req(name: &str) -> Vec<u8> {
    #[derive(Serialize)]
    struct Body {
        log_name: String,
    }

    let mut body = vec![0x00, 0x00];
    let req = serde_cbor::to_vec(&Body {
        log_name: name.into(),
    })
    .unwrap();
    body.extend(req);
    body
}

pub fn new_log_del_req(name: &str) -> Vec<u8> {
    #[derive(Serialize)]
    struct Body {
        log_name: String,
    }

    let mut body = vec![0x00, 0x02];
    let req = serde_cbor::to_vec(&Body {
        log_name: name.into(),
    })
    .unwrap();
    body.extend(req);
    body
}

pub fn new_itr_add_req(name: &str, itr_name: &str, typ: &str) -> Vec<u8> {
    #[derive(Serialize)]
    struct Body {
        log_name: String,
        iterator_name: String,
        iterator_kind: String,
        iterator_func: String,
    }

    let mut body = vec![0x00, 0x05];
    let req = serde_cbor::to_vec(&Body {
        log_name: name.into(),
        iterator_name: itr_name.into(),
        iterator_kind: typ.into(),
        iterator_func: "return msg".into(),
    })
    .unwrap();
    body.extend(req);
    body
}

pub fn new_msg_add_req(name: &str, message: Vec<u8>) -> Vec<u8> {
    #[derive(Serialize)]
    struct Body {
        log_name: String,
        message: serde_cbor::Value,
    }

    let mut body = vec![0x00, 0x04];
    let req = serde_cbor::to_vec(&Body {
        log_name: name.into(),
        message: serde_cbor::Value::Bytes(message),
    })
    .unwrap();
    body.extend(req);
    body
}

pub fn new_log_list_req() -> Vec<u8> {
    vec![0x00, 0x03]
}
pub fn new_itr_list_req() -> Vec<u8> {
    vec![0x00, 0x06]
}
pub fn new_itr_next_req(name: &str, message_id: usize, count: usize) -> Vec<u8> {
    #[derive(Serialize)]
    struct Body {
        iterator_name: String,
        message_id: usize,
        count: usize,
    }

    let mut body = vec![0x00, 0x07];
    let req = serde_cbor::to_vec(&Body {
        iterator_name: name.into(),
        message_id,
        count,
    })
    .unwrap();
    body.extend(req);
    body
}

pub fn send_req(
    framer: &mut Framed<TcpStream, LengthDelimitedCodec>,
    bytes: Vec<u8>,
) -> (u8, u8, Vec<u8>) {
    framer
        .send(Bytes::from(bytes))
        .expect("could not send command");

    let result = framer
        .next()
        .expect("no response from remits")
        .expect("could not understand response");

    (result[0], result[1], result[2..].to_vec())
}

pub fn connect_to_remits() -> Framed<TcpStream, LengthDelimitedCodec> {
    let std_stream = stdTcpStream::connect(LOCAL_REMITS)
        .expect("could not connect to localhost:4242");
    let stream = TcpStream::from_std(std_stream);
    Framed::new(stream, LengthDelimitedCodec::new())
}
