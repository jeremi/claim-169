#!/bin/bash
# Cross-language conformance tests for claim169
# Compares output from Python and TypeScript SDKs to ensure consistency

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_VECTORS="$PROJECT_ROOT/test-vectors"
TEMP_DIR=$(mktemp -d)

cleanup() {
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

echo "=== Claim 169 Cross-Language Conformance Tests ==="
echo ""

# Collect all test vectors
VECTORS_JSON=$(cat << 'EOF'
[]
EOF
)

# Build vectors JSON array
for category in valid invalid edge; do
    for vector_file in "$TEST_VECTORS/$category"/*.json; do
        if [ -f "$vector_file" ]; then
            name=$(basename "$vector_file" .json)
            qr_data=$(python3 -c "import json; print(json.load(open('$vector_file'))['qr_data'])")
            VECTORS_JSON=$(echo "$VECTORS_JSON" | python3 -c "
import sys, json
vectors = json.load(sys.stdin)
vectors.append({'name': '$name', 'category': '$category', 'qr_data': '''$qr_data'''})
print(json.dumps(vectors))
")
        fi
    done
done

# Save vectors to temp file
echo "$VECTORS_JSON" > "$TEMP_DIR/vectors.json"

# Python conformance script
cat > "$TEMP_DIR/python_conformance.py" << 'PYTHON_SCRIPT'
import json
import sys
import claim169

with open(sys.argv[1]) as f:
    vectors = json.load(f)

results = []
for v in vectors:
    try:
        # Disable timestamp validation to match TypeScript defaults
        result = claim169.decode(v['qr_data'], validate_timestamps=False)
        results.append({
            'name': v['name'],
            'category': v['category'],
            'success': True,
            'id': result.claim169.id,
            'fullName': result.claim169.full_name,
            'dateOfBirth': result.claim169.date_of_birth,
            'gender': result.claim169.gender,
            'issuer': result.cwt_meta.issuer,
            'expiresAt': result.cwt_meta.expires_at,
            'verificationStatus': result.verification_status,
        })
    except Exception as e:
        results.append({
            'name': v['name'],
            'category': v['category'],
            'success': False,
            'error': str(e),
        })

print(json.dumps(results, sort_keys=True))
PYTHON_SCRIPT

# TypeScript conformance script (uses vitest for WASM support)
cat > "$PROJECT_ROOT/sdks/typescript/tests/conformance.test.ts" << 'TS_SCRIPT'
import { describe, it, expect } from "vitest";
import { decode } from "../src/index.js";
import * as fs from "fs";

interface Vector {
  name: string;
  category: string;
  qr_data: string;
}

interface Result {
  name: string;
  category: string;
  success: boolean;
  id?: string;
  fullName?: string;
  dateOfBirth?: string;
  gender?: number;
  issuer?: string;
  expiresAt?: number;
  verificationStatus?: string;
  error?: string;
}

const vectorsPath = process.env.VECTORS_PATH || "";
const outputPath = process.env.OUTPUT_PATH || "";

describe("conformance", () => {
  it("processes all vectors", () => {
    const vectors: Vector[] = JSON.parse(fs.readFileSync(vectorsPath, "utf-8"));
    const results: Result[] = [];

    for (const v of vectors) {
      try {
        const result = decode(v.qr_data);
        results.push({
          name: v.name,
          category: v.category,
          success: true,
          id: result.claim169.id,
          fullName: result.claim169.fullName,
          dateOfBirth: result.claim169.dateOfBirth,
          gender: result.claim169.gender,
          issuer: result.cwtMeta.issuer,
          expiresAt: result.cwtMeta.expiresAt,
          verificationStatus: result.verificationStatus,
        });
      } catch (e) {
        results.push({
          name: v.name,
          category: v.category,
          success: false,
          error: (e as Error).message,
        });
      }
    }

    fs.writeFileSync(outputPath, JSON.stringify(results, null, 2));
    expect(results.length).toBe(vectors.length);
  });
});
TS_SCRIPT

# Cleanup function for TS test file
cleanup_ts() {
    rm -f "$PROJECT_ROOT/sdks/typescript/tests/conformance.test.ts"
}

echo "Running Python conformance tests..."
python_results=$(python3 "$TEMP_DIR/python_conformance.py" "$TEMP_DIR/vectors.json")
echo "$python_results" > "$TEMP_DIR/python_results.json"

echo "Running TypeScript conformance tests..."
cd "$PROJECT_ROOT/sdks/typescript"
VECTORS_PATH="$TEMP_DIR/vectors.json" OUTPUT_PATH="$TEMP_DIR/ts_results.json" \
    npm exec -- vitest run tests/conformance.test.ts --reporter=basic 2>&1 | grep -v "^$" || true

# Cleanup TS test file
cleanup_ts

# Compare results
echo ""
echo "Comparing results..."

python3 << COMPARE_SCRIPT
import json

with open("$TEMP_DIR/python_results.json") as f:
    py_results = {r['name']: r for r in json.load(f)}

with open("$TEMP_DIR/ts_results.json") as f:
    ts_results = {r['name']: r for r in json.load(f)}

pass_count = 0
fail_count = 0

for name in sorted(set(py_results.keys()) | set(ts_results.keys())):
    py = py_results.get(name, {'success': None})
    ts = ts_results.get(name, {'success': None})

    # Both should have same success/failure
    if py.get('success') != ts.get('success'):
        print(f"FAIL {name}: py_success={py.get('success')} ts_success={ts.get('success')}")
        if py.get('error'):
            print(f"  Python error: {py.get('error')}")
        if ts.get('error'):
            print(f"  TypeScript error: {ts.get('error')}")
        fail_count += 1
        continue

    # If both succeeded, compare key fields
    if py.get('success') and ts.get('success'):
        mismatches = []
        for field in ['id', 'fullName', 'dateOfBirth', 'gender', 'issuer', 'expiresAt', 'verificationStatus']:
            if py.get(field) != ts.get(field):
                mismatches.append(f"{field}: py={py.get(field)} ts={ts.get(field)}")

        if mismatches:
            print(f"FAIL {name}: field mismatches")
            for m in mismatches:
                print(f"  {m}")
            fail_count += 1
        else:
            print(f"PASS {name}")
            pass_count += 1
    else:
        # Both failed - this is expected for invalid vectors
        print(f"PASS {name} (both rejected)")
        pass_count += 1

print("")
print(f"=== Results ===")
print(f"Passed: {pass_count}")
print(f"Failed: {fail_count}")

if fail_count > 0:
    exit(1)

print("")
print("All conformance tests passed!")
COMPARE_SCRIPT
