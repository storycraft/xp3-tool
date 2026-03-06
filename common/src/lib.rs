use core::{
    pin::{Pin, pin},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};
use std::io::{self, BufRead, Read, Seek, Write};

use tokio::io::ReadBuf;
pub use tokio::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite, copy};

pub fn oneshot_async<Fut: Future>(fut: Fut) -> Fut::Output {
    const VTABLE: RawWakerVTable = RawWakerVTable::new(|_| RAW, |_| {}, |_| {}, |_| {});
    const RAW: RawWaker = RawWaker::new(&(), &VTABLE);
    const WAKER: Waker = unsafe { Waker::from_raw(RAW) };

    match pin!(fut).poll(&mut Context::from_waker(&WAKER)) {
        Poll::Ready(v) => v,
        Poll::Pending => unreachable!(),
    }
}

pub struct SyncIo<T>(pub T);

impl<T: Read + Unpin> AsyncRead for SyncIo<T> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let read = self.0.read(buf.initialize_unfilled())?;
        buf.set_filled(read);
        Poll::Ready(Ok(()))
    }
}

impl<T: BufRead + Unpin> AsyncBufRead for SyncIo<T> {
    fn poll_fill_buf(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
        Poll::Ready(self.get_mut().0.fill_buf())
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        self.get_mut().0.consume(amt);
    }
}

impl<T: Seek + Unpin> AsyncSeek for SyncIo<T> {
    fn start_seek(self: Pin<&mut Self>, position: io::SeekFrom) -> io::Result<()> {
        self.get_mut().0.seek(position)?;
        Ok(())
    }

    fn poll_complete(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<u64>> {
        Poll::Ready(self.get_mut().0.stream_position())
    }
}

impl<T: Write + Unpin> AsyncWrite for SyncIo<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(self.get_mut().0.write(buf))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.get_mut().0.flush()?;
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
