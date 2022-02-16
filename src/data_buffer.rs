use std::str;

/* Byte Array Wrapper */
pub struct DataBuffer {
    buffer: [u8; 4048], // Internal byte array
    next_read: usize, // Next index to read on
}

impl DataBuffer {
    pub fn from(buffer: [u8; 4048]) -> DataBuffer {
        DataBuffer {
            buffer,
            next_read: 0
        }
    }

    pub fn has_next(&self) -> bool {
        self.next_read < self.buffer.len()
    }

    pub fn next(&mut self) -> u8 {
        self.next_read += 1;
        self.buffer[self.next_read - 1]
    }

    // Big endian
    pub fn read_unsigned_short(&mut self, value: &mut u16) -> u32 {
        *value = 0;

        *value |= (self.next() as u16) << 8;
        *value |= self.next() as u16;

        2 // Read 2 bytes
    }

    pub fn read_string(&mut self, value: &mut String) -> u32 {
        let mut bytes_read: u32 = 0;
        let mut string_size: i32 = 0;
        bytes_read += self.read_var_int(&mut string_size);

        let mut unicode_vector = Vec::new();

        for i in 0..string_size {
            unicode_vector.push(self.next());
        }

        let string = str::from_utf8_mut(&mut unicode_vector);

        match string {
            Ok(result) => *value = result.parse().unwrap(), // From slice to string
            Err(error) => *value = String::from(error.to_string()),
        }

        bytes_read += string_size as u32; // Might not be accurate if string wasn't valid
        bytes_read
    }

    // https://wiki.vg/Protocol
    // Little endian
    pub fn read_var_int(&mut self, value: &mut i32) -> u32 {
        let mut length = 0;
        let mut current_byte: u8 = 0;

        for _i in 0..6 {
            if !self.has_next() {
                return 0;
            }

            current_byte = self.next();
            *value |= ((current_byte as i32 & 0x7F) << (length * 7)); // other method without casting?

            length += 1;
            if length > 5 {
                println!("VarInt too big!");
                return 0;
            }

            if (current_byte & 0x80) != 0x80 {
                break;
            }
        }

        length
    }
}