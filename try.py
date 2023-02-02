# a script to generate a file containing a hash
# with almost all types
from copy import deepcopy

import numpy as np

from karabo.native import encodeBinary, Hash, Configurable, Int32


class SimpleSchema(Configurable):
    i32 = Int32()


h = Hash()
h["bool"] = True
h['vbool'] = [False, True]
h["i8"] = np.int8(-1)
h['vi8'] = [np.int8(-1)]
h["u8"] = np.uint8(250)
h["vu8"] = [np.uint8(250)]
h["i16"] = np.int16(-200)
h["vi16"] = [np.int16(-200)]
h["u16"] = np.uint16(599)
h["vu16"] = [np.uint16(599)]
h["i32"] = np.int32(12)
h["vi32"] = [np.int32(12)]
h["u32"] = np.uint32(12)
h["vu32"] = [np.uint32(12)]
h["i64"] = np.int64(-12)
h["vi64"] = [np.int64(-12)]
h["u64"] = np.uint64(12)
h["vu64"] = [np.uint64(12)]
h["f32"] = np.float32(12.)
h["vf32"] = [np.float32(12.)]
h["f64"] = np.float64(200.)
h["vf64"] = [np.float64(12)]
h["string"] = "hi"
h["vstring"] = ["hi"]

out = deepcopy(h)
out["node"] = h
out["vh"] = [h]

out["schema"] = SimpleSchema.getClassSchema()

with open("file.bin", "wb") as f:
    f.write(encodeBinary(out))