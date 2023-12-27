use common::Message;
use defmt::warn;
use postcard::take_from_bytes;

pub struct NetBuffer<const BUF_SIZE: usize, const COPY_BUF_SIZE: usize> {
    pub cursor: usize,
    pub buf: [u8; BUF_SIZE],

    copy_buf: [u8; COPY_BUF_SIZE],
}

impl<const T: usize, const U: usize> NetBuffer<T, U> {
    pub fn process_msgs_ok<F>(&mut self, callback: F) -> bool
    where
        F: Fn(Message),
    {
        let mut to_deserialize = &self.buf[0..self.cursor];

        loop {
            match take_from_bytes::<Message>(to_deserialize) {
                Ok((message, unused)) => {
                    to_deserialize = unused;
                    callback(message);

                    if to_deserialize.is_empty() {
                        self.cursor = 0;
                        return true;
                    }
                }
                Err(_) => {
                    warn!("Could not deserialize buffer, skipping...");

                    let left_over_len = to_deserialize.len();
                    self.copy_buf[0..left_over_len].clone_from_slice(to_deserialize);
                    self.buf[0..left_over_len].clone_from_slice(&self.copy_buf[0..left_over_len]);
                    self.cursor = left_over_len;

                    return false;
                }
            }
        }
    }

    pub fn as_buf(&mut self) -> &mut [u8] {
        &mut self.buf[self.cursor..]
    }
}

impl<const T: usize, const U: usize> Default for NetBuffer<T, U> {
    fn default() -> Self {
        Self {
            cursor: 0,
            buf: [0u8; T],
            copy_buf: [0u8; U],
        }
    }
}
