#!/usr/bin/env python2.7
# vim: fileencoding=utf-8

import ctypes
import os

project_dir = os.path.dirname(os.path.abspath(__file__))

column_names = ['a', 'b', 'c', 'd']
column_spec = ['a', 'b']
csv_path = '/home/padznich/code/rust_/csv_parser/t.csv'


def write_csv_to_postgres(csv_path, column_nmaes, column_spec=None):
    """
    Write data from CSV-file to database
    :param csv_path: [STRING] path to CSV-file
    :param column_nmaes: [LIST] CSV-file header
    :param column_spec: [LIST] columns to write to DB
    :return: [BOOL] True - in case of success, False - when fails
    """
    try:
        # if column_spec not specified take all columns
        if not column_spec:
            column_spec = column_names

        # Transform to C-array
        column_names_c = (ctypes.c_char_p * len(column_names))(*column_names)
        column_spec_c = (ctypes.c_char_p * len(column_spec))(*column_spec)

        # Get rust library
        lib = ctypes.cdll.LoadLibrary(os.path.join(project_dir, 'target/release/libcsv_parser.so'))

        # Write to DB through RUST
        lib.write_to_postgres(csv_path, column_names_c, len(column_names), column_spec_c, len(column_spec))

        return True

    except Exception as ex:
        print ex
        return False


write_csv_to_postgres(csv_path, column_names, column_spec)


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
