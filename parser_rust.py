#!/usr/bin/env python2.7
# vim: fileencoding=utf-8

import ctypes

column_names = ['a', 'b', 'c', 'd']
# Transform column names to C-array
column_names_c = (ctypes.c_char_p * len(column_names))(*column_names)

lib = ctypes.cdll.LoadLibrary('target/release/libcsv_parser.so')
lib.parse('/home/padznich/code/rust_/csv_parser/t.csv', column_names_c, len(column_names))


# list_to_send = ['t.csv', 'm.csv']
# c_array = (ctypes.c_char_p * len(list_to_send))(*list_to_send)
# lib.parse_many(c_array, len(list_to_send))
# print('Done')
#
#
# def theme_song_generate(count):
#
#     lib.theme_song_generate.argtypes = (ctypes.c_uint8,)
#     lib.theme_song_generate.restype = ctypes.c_void_p
#     lib.theme_song_free.argtypes = (ctypes.c_void_p,)
#     ptr = lib.theme_song_generate(count)
#     try:
#         return ctypes.cast(ptr, ctypes.c_char_p).value.decode('utf-8')
#     finally:
#         lib.theme_song_free(ptr)
#
# print(theme_song_generate(5)) 
