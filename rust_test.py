from rust_python_talk_2025 import histogram_256_from_file as histogram_256

if __name__ == "__main__":
    # Let the Rust extension read and parse the file, returning the histogram.
    hist = histogram_256("histogram_1m.txt")
    print(hist)
