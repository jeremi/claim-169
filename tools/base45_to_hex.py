"""Decode Base45 data, decompress with zlib, and save as hex."""

import sys
import zlib

BASE45_CHARSET = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:"


def base45_decode(data: str) -> bytes:
    result = []
    stripped = data.strip()
    i = 0
    while i < len(stripped):
        if i + 2 < len(stripped):
            c, d, e = (
                BASE45_CHARSET.index(stripped[i]),
                BASE45_CHARSET.index(stripped[i + 1]),
                BASE45_CHARSET.index(stripped[i + 2]),
            )
            value = c + d * 45 + e * 45 * 45
            result.append(value >> 8)
            result.append(value & 0xFF)
            i += 3
        elif i + 1 < len(stripped):
            c, d = (
                BASE45_CHARSET.index(stripped[i]),
                BASE45_CHARSET.index(stripped[i + 1]),
            )
            result.append(c + d * 45)
            i += 2
        else:
            raise ValueError(f"Invalid Base45 input: trailing character at position {i}")
    return bytes(result)


def main() -> None:
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <input_file> <output_file>")
        print("  input_file:  file containing Base45 encoded data")
        print("  output_file: file to write hex encoded output")
        sys.exit(1)

    input_path = sys.argv[1]
    output_path = sys.argv[2]

    with open(input_path) as f:
        base45_data = f.read()

    raw = base45_decode(base45_data)
    decompressed = zlib.decompress(raw)

    with open(output_path, "w") as f:
        f.write(decompressed.hex())

    print(f"Decoded {len(base45_data)} Base45 chars -> {len(raw)} compressed bytes -> {len(decompressed)} decompressed bytes")
    print(f"Hex output written to {output_path}")


if __name__ == "__main__":
    main()
