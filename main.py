import ctypes

mylib = ctypes.CDLL("./target/debug/libfact.so")
mylib.fact.argtypes = (ctypes.c_uint,)
mylib.fact.restype = ctypes.c_uint
print(mylib.fact(4))
