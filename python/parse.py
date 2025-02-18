# Here's an example on how to parse the output from the indexer
# We're aiming to read the first 21 values from the height_to_timestamp vec

import sys
# import struct
import datetime

with open("../_outputs/indexes/vecs/height_to_timestamp/vec", "rb") as file:
    for x in range(0, 21):
        b = file.read(4) # Need to check the rust side to find the size, at least for now
        number = int.from_bytes(b, sys.byteorder)
        date = datetime.date.fromtimestamp(number)
        print(date)

# print(int.from_bytes([21], sys.byteorder)) # 21 u8 native endian
# print(int.from_bytes([21, 0], sys.byteorder)) # 21 u16 native endian
# print(int.from_bytes([21, 0, 0, 0], sys.byteorder)) # 21 u32 native endian
# print(int.from_bytes([21, 0, 0, 0, 0, 0, 0, 0], sys.byteorder)) # 21 u64/usize native endian

# # check i8, ...

# print(struct.unpack('f', bytes([0, 0, 168, 65]))) # 21.0 f32 native endian
# print(struct.unpack('d', bytes([0, 0, 0, 0, 0, 0, 53, 64]))) # 21.0 f64 native endian
# print(struct.unpack('<f', bytes([0, 0, 168, 65]))) # 21.0 f32 little endian
# print(struct.unpack('<d', bytes([0, 0, 0, 0, 0, 0, 53, 64]))) # 21.0 f64 little endian
# print(struct.unpack('>f', bytes([65, 168, 0, 0]))) # 21.0 f32 big endian
# print(struct.unpack('>d', bytes([64, 53, 0, 0, 0, 0, 0, 0]))) # 21.0 f64 big endian
