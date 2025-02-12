# Here's an example on how to parse the output from the indexer
# We're aiming to read the first 21 values from the height_to_timestamp vec

import sys
import datetime

with open("../_outputs/indexes/vecs/height_to_timestamp/vec", "rb") as file:
    for x in range(0, 21):
        bytes = file.read(4) # Need to check the rust side to find the size, at least for now
        number = int.from_bytes(bytes, sys.byteorder)
        date = datetime.date.fromtimestamp(number)
        print(date)
