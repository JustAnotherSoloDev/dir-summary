import ctypes


import ctypes
from ctypes import c_char_p

# Load the shared library
lib = ctypes.CDLL('./target/release/dirSummary.dll')  # Adjust for your OS

# Define return types and argument types
# lib.generate_message.restype = c_char_p
# lib.generate_message.argtypes = [ctypes.c_bool]

# lib.create_hashmap.restype = c_char_p
# lib.create_hashmap.argtypes = []

lib.create_report.restype = c_char_p
lib.create_report.argtypes = [c_char_p]

lib.directory_summary.restype = c_char_p
lib.directory_summary.argtypes = [c_char_p,c_char_p]

lib.summary.restype = c_char_p
lib.summary.argtypes = [c_char_p]

def main():
    str1 = "D:\\github\\dir-summary\\target".encode('utf-8')
    report_path = lib.create_report(str1)
    print("create result:", report_path)
    summary=lib.directory_summary(report_path,str1)
    print("Summary:",summary)
    dir_summary=lib.summary(report_path)
    print("directory report",dir_summary)
if __name__ == '__main__':
    main()