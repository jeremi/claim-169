#!/bin/bash
# Cross-language conformance tests for claim169
# Compares output from Python and TypeScript SDKs to ensure consistency

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_VECTORS="$PROJECT_ROOT/test-vectors"
TEMP_DIR=$(mktemp -d)

trap "rm -rf $TEMP_DIR" EXIT

echo "=== Claim 169 Cross-Language Conformance Tests ==="
echo ""

# Python decode script
cat > "$TEMP_DIR/python_decode.py" << 'PYTHON_SCRIPT'
import sys
import json
import claim169

def decode_vector(qr_data):
    try:
        result = claim169.decode(qr_data)
        return {
            "success": True,
            "claim169": result.claim169.to_dict(),
            "cwtMeta": {
                "issuer": result.cwt_meta.issuer,
                "subject": result.cwt_meta.subject,
                "expiresAt": result.cwt_meta.expires_at,
                "notBefore": result.cwt_meta.not_before,
                "issuedAt": result.cwt_meta.issued_at,
            },
            "verificationStatus": result.verification_status,
        }
    except Exception as e:
        return {
            "success": False,
            "error": str(e),
        }

if __name__ == "__main__":
    qr_data = sys.argv[1]
    result = decode_vector(qr_data)
    print(json.dumps(result, sort_keys=True))
PYTHON_SCRIPT

# TypeScript decode script
cat > "$TEMP_DIR/ts_decode.mjs" << 'TS_SCRIPT'
import { decode } from '../sdks/typescript/dist/index.js';

const qrData = process.argv[2];

function decodeVector(qrData) {
    try {
        const result = decode(qrData);
        return {
            success: true,
            claim169: result.claim169,
            cwtMeta: result.cwtMeta,
            verificationStatus: result.verificationStatus,
        };
    } catch (e) {
        return {
            success: false,
            error: e.message,
        };
    }
}

const result = decodeVector(qrData);
console.log(JSON.stringify(result, Object.keys(result).sort()));
TS_SCRIPT

PASS_COUNT=0
FAIL_COUNT=0

compare_outputs() {
    local name="$1"
    local qr_data="$2"

    echo -n "Testing $name... "

    # Run Python
    python_output=$(python3 "$TEMP_DIR/python_decode.py" "$qr_data" 2>&1) || true

    # Run TypeScript
    ts_output=$(node "$TEMP_DIR/ts_decode.mjs" "$qr_data" 2>&1) || true

    # Check if both succeeded or both failed
    python_success=$(echo "$python_output" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('success', False))" 2>/dev/null || echo "False")
    ts_success=$(echo "$ts_output" | node -e "const d=JSON.parse(require('fs').readFileSync(0,'utf-8'));console.log(d.success||false)" 2>/dev/null || echo "false")

    if [[ "$python_success" == "True" && "$ts_success" == "true" ]]; then
        # Both succeeded - compare claim169 fields and CWT metadata
        python_id=$(echo "$python_output" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('claim169',{}).get('id',''))" 2>/dev/null || echo "")
        ts_id=$(echo "$ts_output" | node -e "const d=JSON.parse(require('fs').readFileSync(0,'utf-8'));console.log(d.claim169?.id||'')" 2>/dev/null || echo "")

        python_name=$(echo "$python_output" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('claim169',{}).get('fullName',''))" 2>/dev/null || echo "")
        ts_name=$(echo "$ts_output" | node -e "const d=JSON.parse(require('fs').readFileSync(0,'utf-8'));console.log(d.claim169?.fullName||'')" 2>/dev/null || echo "")

        python_dob=$(echo "$python_output" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('claim169',{}).get('dateOfBirth',''))" 2>/dev/null || echo "")
        ts_dob=$(echo "$ts_output" | node -e "const d=JSON.parse(require('fs').readFileSync(0,'utf-8'));console.log(d.claim169?.dateOfBirth||'')" 2>/dev/null || echo "")

        python_gender=$(echo "$python_output" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('claim169',{}).get('gender',''))" 2>/dev/null || echo "")
        ts_gender=$(echo "$ts_output" | node -e "const d=JSON.parse(require('fs').readFileSync(0,'utf-8'));console.log(d.claim169?.gender||'')" 2>/dev/null || echo "")

        python_issuer=$(echo "$python_output" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('cwtMeta',{}).get('issuer',''))" 2>/dev/null || echo "")
        ts_issuer=$(echo "$ts_output" | node -e "const d=JSON.parse(require('fs').readFileSync(0,'utf-8'));console.log(d.cwtMeta?.issuer||'')" 2>/dev/null || echo "")

        python_expires=$(echo "$python_output" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('cwtMeta',{}).get('expiresAt',''))" 2>/dev/null || echo "")
        ts_expires=$(echo "$ts_output" | node -e "const d=JSON.parse(require('fs').readFileSync(0,'utf-8'));console.log(d.cwtMeta?.expiresAt||'')" 2>/dev/null || echo "")

        python_vstatus=$(echo "$python_output" | python3 -c "import sys, json; d=json.load(sys.stdin); print(d.get('verificationStatus',''))" 2>/dev/null || echo "")
        ts_vstatus=$(echo "$ts_output" | node -e "const d=JSON.parse(require('fs').readFileSync(0,'utf-8'));console.log(d.verificationStatus||'')" 2>/dev/null || echo "")

        if [[ "$python_id" == "$ts_id" && "$python_name" == "$ts_name" && "$python_dob" == "$ts_dob" && "$python_gender" == "$ts_gender" && "$python_issuer" == "$ts_issuer" && "$python_expires" == "$ts_expires" && "$python_vstatus" == "$ts_vstatus" ]]; then
            echo "PASS (id=$python_id)"
            ((PASS_COUNT++))
        else
            echo "FAIL (mismatch)"
            echo "  id: py=$python_id ts=$ts_id"
            echo "  fullName: py=$python_name ts=$ts_name"
            echo "  dateOfBirth: py=$python_dob ts=$ts_dob"
            echo "  gender: py=$python_gender ts=$ts_gender"
            echo "  cwtMeta.issuer: py=$python_issuer ts=$ts_issuer"
            echo "  cwtMeta.expiresAt: py=$python_expires ts=$ts_expires"
            echo "  verificationStatus: py=$python_vstatus ts=$ts_vstatus"
            ((FAIL_COUNT++))
        fi
    elif [[ "$python_success" == "False" && "$ts_success" == "false" ]]; then
        # Both failed - this is expected for invalid vectors
        echo "PASS (both rejected)"
        ((PASS_COUNT++))
    else
        echo "FAIL (py_success=$python_success ts_success=$ts_success)"
        ((FAIL_COUNT++))
    fi
}

echo "--- Valid Vectors ---"
for vector_file in "$TEST_VECTORS/valid"/*.json; do
    name=$(basename "$vector_file" .json)
    qr_data=$(python3 -c "import json; print(json.load(open('$vector_file'))['qr_data'])")
    compare_outputs "$name" "$qr_data"
done

echo ""
echo "--- Invalid Vectors ---"
for vector_file in "$TEST_VECTORS/invalid"/*.json; do
    name=$(basename "$vector_file" .json)
    qr_data=$(python3 -c "import json; print(json.load(open('$vector_file'))['qr_data'])")
    compare_outputs "$name" "$qr_data"
done

echo ""
echo "--- Edge Case Vectors ---"
for vector_file in "$TEST_VECTORS/edge"/*.json; do
    name=$(basename "$vector_file" .json)
    qr_data=$(python3 -c "import json; print(json.load(open('$vector_file'))['qr_data'])")
    compare_outputs "$name" "$qr_data"
done

echo ""
echo "=== Results ==="
echo "Passed: $PASS_COUNT"
echo "Failed: $FAIL_COUNT"

if [[ $FAIL_COUNT -gt 0 ]]; then
    exit 1
fi

echo ""
echo "All conformance tests passed!"
