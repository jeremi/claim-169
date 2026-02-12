"""Inspect a Claim 169 QR code: Base45 decode, zlib decompress, and parse CBOR/COSE structure."""

import sys
import zlib
from datetime import datetime, timezone

BASE45_CHARSET = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:"

# COSE algorithm labels (IANA registry)
COSE_ALGORITHMS = {
    -8: "EdDSA",
    -7: "ES256",
    -35: "ES384",
    -36: "ES512",
    1: "A128GCM",
    3: "A256GCM",
}

# CWT claim keys (RFC 8392)
CWT_CLAIM_NAMES = {
    1: "iss",
    2: "sub",
    3: "aud",
    4: "exp",
    5: "nbf",
    6: "iat",
    7: "cti",
    169: "claim-169",
}

# Claim 169 field keys
CLAIM169_FIELD_NAMES = {
    1: "id",
    2: "version",
    3: "language",
    4: "full_name",
    5: "first_name",
    6: "middle_name",
    7: "last_name",
    8: "date_of_birth",
    9: "gender",
    10: "address",
    11: "email",
    12: "phone",
    13: "nationality",
    14: "marital_status",
    15: "guardian",
    16: "photo",
    17: "photo_format",
    18: "best_quality_fingers",
    19: "secondary_full_name",
    20: "secondary_language",
    21: "location_code",
    22: "legal_status",
    23: "country_of_issuance",
    50: "right_thumb",
    51: "right_pointer_finger",
    52: "right_middle_finger",
    53: "right_ring_finger",
    54: "right_little_finger",
    55: "left_thumb",
    56: "left_pointer_finger",
    57: "left_middle_finger",
    58: "left_ring_finger",
    59: "left_little_finger",
    60: "right_iris",
    61: "left_iris",
    62: "face",
    63: "right_palm",
    64: "left_palm",
    65: "voice",
}

GENDER_NAMES = {1: "Male", 2: "Female", 3: "Other"}
MARITAL_NAMES = {1: "Unmarried", 2: "Married", 3: "Divorced"}
PHOTO_FORMAT_NAMES = {1: "JPEG", 2: "JPEG2000", 3: "AVIF", 4: "WebP"}
BIOMETRIC_FORMAT_NAMES = {0: "Image", 1: "Template", 2: "Sound", 3: "BioHash"}


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


class CborParser:
    """Minimal CBOR parser sufficient for COSE/CWT/Claim 169 inspection."""

    def __init__(self, data: bytes) -> None:
        self.data = data
        self.pos = 0

    def _read(self, count: int) -> bytes:
        if self.pos + count > len(self.data):
            raise ValueError(f"unexpected end of data at offset {self.pos}")
        chunk = self.data[self.pos : self.pos + count]
        self.pos += count
        return chunk

    def _read_head(self) -> tuple[int, int]:
        """Read a CBOR head byte and return (major_type, argument)."""
        b = self._read(1)[0]
        major = (b >> 5) & 0x07
        additional = b & 0x1F
        if additional < 24:
            return major, additional
        elif additional == 24:
            return major, self._read(1)[0]
        elif additional == 25:
            return major, int.from_bytes(self._read(2), "big")
        elif additional == 26:
            return major, int.from_bytes(self._read(4), "big")
        elif additional == 27:
            return major, int.from_bytes(self._read(8), "big")
        else:
            raise ValueError(f"unsupported CBOR additional info {additional} at offset {self.pos - 1}")

    def parse(self) -> object:
        """Parse one CBOR data item and return a Python object."""
        major, arg = self._read_head()
        if major == 0:  # unsigned int
            return arg
        elif major == 1:  # negative int
            return -1 - arg
        elif major == 2:  # byte string
            return self._read(arg)
        elif major == 3:  # text string
            return self._read(arg).decode("utf-8", errors="replace")
        elif major == 4:  # array
            return [self.parse() for _ in range(arg)]
        elif major == 5:  # map
            result = {}
            for _ in range(arg):
                key = self.parse()
                value = self.parse()
                result[key] = value
            return result
        elif major == 6:  # tag
            tag_number = arg
            value = self.parse()
            return ("tag", tag_number, value)
        elif major == 7:  # simple/float
            if arg == 20:
                return False
            elif arg == 21:
                return True
            elif arg == 22:
                return None
            else:
                return ("simple", arg)
        else:
            raise ValueError(f"unknown CBOR major type {major}")


def format_timestamp(ts: int) -> str:
    """Format a Unix timestamp as a human-readable string."""
    try:
        dt = datetime.fromtimestamp(ts, tz=timezone.utc)
        return f"{ts} ({dt.strftime('%Y-%m-%d %H:%M:%S UTC')})"
    except (ValueError, OSError, OverflowError):
        return str(ts)


def format_bytes_summary(data: bytes, max_show: int = 16) -> str:
    """Format a byte string as a truncated hex summary."""
    if len(data) <= max_show:
        return data.hex()
    return f"{data[:max_show].hex()}... ({len(data)} bytes)"


def print_cose_headers(protected_bytes: bytes, unprotected: dict, indent: str = "  ") -> None:
    """Print COSE protected and unprotected headers."""
    if protected_bytes:
        try:
            parser = CborParser(protected_bytes)
            protected = parser.parse()
            if isinstance(protected, dict):
                print(f"{indent}Protected headers:")
                for k, v in protected.items():
                    if k == 1:
                        alg_name = COSE_ALGORITHMS.get(v, f"unknown({v})")
                        print(f"{indent}  alg(1): {v} ({alg_name})")
                    elif k == 4:
                        print(f"{indent}  kid(4): {format_bytes_summary(v) if isinstance(v, bytes) else v}")
                    else:
                        print(f"{indent}  {k}: {format_bytes_summary(v) if isinstance(v, bytes) else v}")
            else:
                print(f"{indent}Protected headers (raw): {protected}")
        except Exception as exc:
            print(f"{indent}Protected headers (parse error: {exc}): {format_bytes_summary(protected_bytes)}")
    else:
        print(f"{indent}Protected headers: (empty)")

    if isinstance(unprotected, dict) and unprotected:
        print(f"{indent}Unprotected headers:")
        for k, v in unprotected.items():
            if k == 4:
                print(f"{indent}  kid(4): {format_bytes_summary(v) if isinstance(v, bytes) else v}")
            else:
                print(f"{indent}  {k}: {format_bytes_summary(v) if isinstance(v, bytes) else v}")
    elif isinstance(unprotected, dict):
        print(f"{indent}Unprotected headers: (empty)")
    else:
        print(f"{indent}Unprotected headers (non-standard): {unprotected}")


def print_cwt_claims(claims: dict, indent: str = "  ") -> None:
    """Print CWT claims with readable names."""
    for k, v in claims.items():
        name = CWT_CLAIM_NAMES.get(k, str(k)) if isinstance(k, int) else str(k)
        if isinstance(k, int) and k in (4, 5, 6) and isinstance(v, int):
            print(f"{indent}{name}({k}): {format_timestamp(v)}")
        elif isinstance(k, str) and k in ("exp", "nbf", "iat") and isinstance(v, int):
            print(f"{indent}{k}: {format_timestamp(v)}")
        elif k == 169 or k == "169":
            if isinstance(v, bytes):
                print(f"{indent}{name}({k}): bstr ({len(v)} bytes) — parsing inner CBOR...")
                try:
                    inner_parser = CborParser(v)
                    inner = inner_parser.parse()
                    if isinstance(inner, dict):
                        print_claim169_fields(inner, indent + "  ")
                    else:
                        print(f"{indent}  (unexpected type: {type(inner).__name__})")
                except Exception as exc:
                    print(f"{indent}  (parse error: {exc})")
            elif isinstance(v, dict):
                print(f"{indent}{name}({k}): map ({len(v)} entries)")
                print_claim169_fields(v, indent + "  ")
            else:
                print(f"{indent}{name}({k}): {v}")
        elif isinstance(v, bytes):
            print(f"{indent}{name}({k}): {format_bytes_summary(v)}")
        else:
            print(f"{indent}{name}({k}): {v}")


def print_claim169_fields(fields: dict, indent: str = "    ") -> None:
    """Print Claim 169 identity fields with readable names."""
    for k, v in fields.items():
        name = CLAIM169_FIELD_NAMES.get(k, str(k)) if isinstance(k, int) else str(k)
        label = f"{name}({k})" if isinstance(k, int) else name

        if isinstance(k, int) and k == 9 and isinstance(v, int):
            print(f"{indent}{label}: {v} ({GENDER_NAMES.get(v, 'unknown')})")
        elif isinstance(k, int) and k == 14 and isinstance(v, int):
            print(f"{indent}{label}: {v} ({MARITAL_NAMES.get(v, 'unknown')})")
        elif isinstance(k, int) and k == 17 and isinstance(v, int):
            print(f"{indent}{label}: {v} ({PHOTO_FORMAT_NAMES.get(v, 'unknown')})")
        elif isinstance(k, int) and k == 16 and isinstance(v, bytes):
            print(f"{indent}{label}: {format_bytes_summary(v)}")
        elif isinstance(k, int) and 50 <= k <= 65:
            # Biometric field
            if isinstance(v, list):
                print(f"{indent}{label}: [{len(v)} entries]")
                for i, entry in enumerate(v):
                    if isinstance(entry, dict):
                        fmt = entry.get(1, entry.get("format", "?"))
                        fmt_name = BIOMETRIC_FORMAT_NAMES.get(fmt, str(fmt))
                        data = entry.get(0, entry.get("data", b""))
                        data_len = len(data) if isinstance(data, bytes) else "?"
                        print(f"{indent}  [{i}] format={fmt_name}, data={data_len} bytes")
                    else:
                        print(f"{indent}  [{i}] {type(entry).__name__}")
            else:
                print(f"{indent}{label}: {v}")
        elif isinstance(v, bytes):
            print(f"{indent}{label}: {format_bytes_summary(v)}")
        elif isinstance(v, str) and len(v) > 60:
            print(f"{indent}{label}: \"{v[:60]}...\" ({len(v)} chars)")
        else:
            print(f"{indent}{label}: {v}")


def inspect(data: str) -> None:
    """Run the full inspection pipeline on Base45-encoded QR data."""
    # Stage 1: Base45 decode
    print("=" * 60)
    print("Stage 1: Base45 Decode")
    print("=" * 60)
    try:
        raw = base45_decode(data)
        print(f"  Input:  {len(data)} characters")
        print(f"  Output: {len(raw)} bytes")
    except ValueError as exc:
        print(f"  FAILED: {exc}")
        return

    # Stage 2: zlib decompress
    print()
    print("=" * 60)
    print("Stage 2: zlib Decompress")
    print("=" * 60)
    try:
        decompressed = zlib.decompress(raw)
        ratio = len(raw) / len(decompressed) * 100 if decompressed else 0
        print(f"  Input:  {len(raw)} bytes")
        print(f"  Output: {len(decompressed)} bytes ({ratio:.0f}% compression ratio)")
    except zlib.error as exc:
        print(f"  FAILED: {exc}")
        print(f"  First 16 bytes: {raw[:16].hex()}")
        return

    # Stage 3: COSE structure
    print()
    print("=" * 60)
    print("Stage 3: COSE Structure")
    print("=" * 60)
    try:
        parser = CborParser(decompressed)
        structure = parser.parse()
    except Exception as exc:
        print(f"  FAILED to parse CBOR: {exc}")
        print(f"  First 32 bytes: {decompressed[:32].hex()}")
        return

    # Check for COSE tag
    tagged = False
    tag_number = None
    if isinstance(structure, tuple) and len(structure) == 3 and structure[0] == "tag":
        tagged = True
        tag_number = structure[1]
        structure = structure[2]
        tag_names = {18: "COSE_Sign1", 16: "COSE_Encrypt0"}
        print(f"  CBOR tag: {tag_number} ({tag_names.get(tag_number, 'unknown')})")
    else:
        print("  CBOR tag: (none — untagged)")

    if not isinstance(structure, list) or len(structure) != 4:
        print(f"  UNEXPECTED: expected array(4), got {type(structure).__name__}")
        if isinstance(structure, list):
            print(f"  Array length: {len(structure)}")
        return

    # Determine COSE type from structure
    protected_raw = structure[0]
    unprotected = structure[1]
    payload_raw = structure[2]
    sig_or_tag = structure[3]

    is_standard_cose = isinstance(protected_raw, bytes) and isinstance(unprotected, dict)

    if is_standard_cose:
        if tag_number == 16:
            print("  Type: COSE_Encrypt0 (standard)")
        elif tag_number == 18:
            print("  Type: COSE_Sign1 (standard)")
        else:
            print("  Type: COSE_Sign1 (untagged, standard structure)")
        print()
        print_cose_headers(protected_raw, unprotected)
    else:
        print("  Type: NON-STANDARD structure")
        print()
        print(f"  [0] protected: {type(protected_raw).__name__} = {protected_raw!r}")
        print(f"  [1] unprotected: {type(unprotected).__name__} = {unprotected!r}")
        if is_standard_cose is False:
            print()
            print("  WARNING: This is not a valid COSE_Sign1 (RFC 9052).")
            print("    Expected: [bstr, map, bstr/nil, bstr]")
            actual_types = [type(x).__name__ for x in structure]
            print(f"    Got:      [{', '.join(actual_types)}]")

    # Payload
    print()
    print(f"  Payload: {len(payload_raw)} bytes" if isinstance(payload_raw, bytes) else f"  Payload: {type(payload_raw).__name__}")
    print(f"  Signature/Tag: {len(sig_or_tag)} bytes" if isinstance(sig_or_tag, bytes) else f"  Signature/Tag: {type(sig_or_tag).__name__}")

    # Stage 4: CWT claims
    print()
    print("=" * 60)
    print("Stage 4: CWT Claims")
    print("=" * 60)
    if isinstance(payload_raw, bytes):
        try:
            payload_parser = CborParser(payload_raw)
            claims = payload_parser.parse()
        except Exception as exc:
            print(f"  FAILED to parse payload CBOR: {exc}")
            print(f"  First 32 bytes: {payload_raw[:32].hex()}")
            return
    else:
        print(f"  Payload is not a byte string (type: {type(payload_raw).__name__})")
        return

    if not isinstance(claims, dict):
        print(f"  UNEXPECTED: expected map, got {type(claims).__name__}")
        return

    uses_text_keys = any(isinstance(k, str) for k in claims)
    uses_int_keys = any(isinstance(k, int) for k in claims)
    if uses_text_keys and not uses_int_keys:
        print("  WARNING: CWT claims use text keys (non-standard).")
        print("    Expected integer keys per RFC 8392 (1=iss, 4=exp, 169=claim-169).")
        print()
    elif uses_text_keys and uses_int_keys:
        print("  WARNING: CWT claims use a mix of text and integer keys.")
        print()

    print_cwt_claims(claims)

    # Stage 5: Claim 169 identity data
    claim169_data = claims.get(169, claims.get("169"))
    if claim169_data is not None and not isinstance(claim169_data, (bytes, dict)):
        print()
        print("=" * 60)
        print("Stage 5: Claim 169 Identity Data")
        print("=" * 60)
        print(f"  Unexpected type for claim 169: {type(claim169_data).__name__}")


def main() -> None:
    if len(sys.argv) == 2:
        input_path = sys.argv[1]
        with open(input_path) as f:
            data = f.read().strip()
    elif len(sys.argv) == 1:
        print("Reading Base45 data from stdin...")
        data = sys.stdin.read().strip()
    else:
        print(f"Usage: {sys.argv[0]} [input_file]")
        print("  Reads Base45 QR data from file or stdin.")
        sys.exit(1)

    if not data:
        print("Error: no input data")
        sys.exit(1)

    inspect(data)


if __name__ == "__main__":
    main()
