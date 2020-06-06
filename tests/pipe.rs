use tokio;
use tokio::runtime;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use quickcheck_macros::quickcheck;
use sluice::pipe::pipe;
use std::io;

#[test]
fn read_empty() {
    let mut rt = runtime::Builder::new()
        .enable_io()
        .build()
        .unwrap();

    rt.block_on(async {
        let (mut reader, writer) = pipe();
        drop(writer);

        let mut out = String::new();
        reader.read_to_string(&mut out).await.unwrap();
        assert_eq!(out, "");
    })
}

#[test]
fn read_then_write() {
    let mut rt = runtime::Builder::new()
        .enable_io()
        .build()
        .unwrap();

    rt.block_on(async {
        let (mut reader, mut writer) = pipe();

        writer.write_all(b"hello world").await.unwrap();

        let mut dest: [u8; 6] = [0; 6];

        assert_eq!(reader.read(&mut dest).await.unwrap(), 6);
        assert_eq!(&dest, b"hello ");

        assert_eq!(reader.read(&mut dest).await.unwrap(), 5);
        assert_eq!(&dest[..5], b"world");
    })
}

#[test]
fn reader_still_drainable_after_writer_disconnects() {
    let mut rt = runtime::Builder::new()
        .enable_io()
        .build()
        .unwrap();

    rt.block_on(async {
        let (mut reader, mut writer) = pipe();

        writer.write_all(b"hello").await.unwrap();

        drop(writer);

        let mut dest: [u8; 5] = [0; 5];
        assert_eq!(reader.read(&mut dest).await.unwrap(), 5);
        assert_eq!(&dest, b"hello");

        // Continue returning Ok(0) forever.
        for _ in 0..3 {
            assert_eq!(reader.read(&mut dest).await.unwrap(), 0);
        }
    })
}

#[test]
fn writer_errors_if_reader_is_dropped() {
    let mut rt = runtime::Builder::new()
        .enable_io()
        .build()
        .unwrap();

    rt.block_on(async {
        let (reader, mut writer) = pipe();

        drop(reader);

        for _ in 0..3 {
            assert_eq!(writer.write(b"hello").await.unwrap_err().kind(), io::ErrorKind::BrokenPipe);
        }
    })
}

#[quickcheck]
fn read_write_chunks_random(chunks: u16) {
    let mut rt = runtime::Builder::new()
        .enable_io()
        .build()
        .unwrap();

    rt.block_on(async {
        let data: [u8; 8192] = [0; 8192];
        let (mut reader, mut writer) = pipe();

        tokio::join!(
            async {
                for chunk in 0..chunks {
                    writer.write_all(&data).await.unwrap();
                }
            },
            async {
                for chunk in 0..chunks {
                    let mut buf = data.clone();
                    reader.read(&mut buf).await.unwrap();
                    assert_eq!(&buf[..], &data[..]);
                }
            },
        );
    })
}
