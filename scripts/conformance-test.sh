#!/bin/bash
# Cross-language conformance tests for claim169
# Compares output from Python, TypeScript, and Kotlin SDKs to ensure consistency

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
        # Disable timestamp validation to avoid time-dependent differences in conformance runs.
        result = claim169.decode(v['qr_data'], validate_timestamps=False, allow_unverified=True)
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
        // Disable timestamp validation to avoid time-dependent differences in conformance runs.
        const result = decode(v.qr_data, {
          allowUnverified: true,
          validateTimestamps: false,
        });
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
    npm exec -- vitest run tests/conformance.test.ts --reporter=dot 2>&1 | grep -v "^$" || true

# Cleanup TS test file
cleanup_ts

# Kotlin conformance: run via Gradle test with conformance output path
echo "Running Kotlin conformance tests..."
cd "$PROJECT_ROOT/sdks/kotlin"
./gradlew :claim169-core:cleanTest :claim169-core:test \
    --tests "org.acn.claim169.ConformanceTest" \
    -Dconformance.vectors.path="$TEMP_DIR/vectors.json" \
    -Dconformance.output.path="$TEMP_DIR/kt_results.json" \
    2>&1 | grep -v "^$" || true
cd "$PROJECT_ROOT"

# If Kotlin output was not generated (e.g., no JDK), skip it in comparison
KT_AVAILABLE=false
if [ -f "$TEMP_DIR/kt_results.json" ]; then
    KT_AVAILABLE=true
fi

# Compare results
echo ""
echo "Comparing results..."

python3 << COMPARE_SCRIPT
import json
import os

with open("$TEMP_DIR/python_results.json") as f:
    py_results = {r['name']: r for r in json.load(f)}

with open("$TEMP_DIR/ts_results.json") as f:
    ts_results = {r['name']: r for r in json.load(f)}

kt_results = {}
kt_available = os.path.exists("$TEMP_DIR/kt_results.json")
if kt_available:
    with open("$TEMP_DIR/kt_results.json") as f:
        kt_results = {r['name']: r for r in json.load(f)}

sdks = {'Python': py_results, 'TypeScript': ts_results}
if kt_available:
    sdks['Kotlin'] = kt_results

all_names = set()
for results in sdks.values():
    all_names.update(results.keys())

pass_count = 0
fail_count = 0

for name in sorted(all_names):
    results_by_sdk = {sdk: results.get(name, {'success': None}) for sdk, results in sdks.items()}

    # All SDKs should agree on success/failure
    successes = {sdk: r.get('success') for sdk, r in results_by_sdk.items() if r.get('success') is not None}
    unique_successes = set(successes.values())

    if len(unique_successes) > 1:
        print(f"FAIL {name}: success mismatch {dict(successes)}")
        for sdk, r in results_by_sdk.items():
            if r.get('error'):
                print(f"  {sdk} error: {r.get('error')}")
        fail_count += 1
        continue

    # If all succeeded, compare key fields across all pairs
    if all(r.get('success') for r in results_by_sdk.values()):
        mismatches = []
        sdk_names = list(results_by_sdk.keys())
        ref_sdk = sdk_names[0]
        ref = results_by_sdk[ref_sdk]

        for field in ['id', 'fullName', 'dateOfBirth', 'gender', 'issuer', 'expiresAt', 'verificationStatus']:
            ref_val = ref.get(field)
            for other_sdk in sdk_names[1:]:
                other_val = results_by_sdk[other_sdk].get(field)
                if ref_val != other_val:
                    mismatches.append(f"{field}: {ref_sdk}={ref_val} {other_sdk}={other_val}")

        if mismatches:
            print(f"FAIL {name}: field mismatches")
            for m in mismatches:
                print(f"  {m}")
            fail_count += 1
        else:
            print(f"PASS {name}")
            pass_count += 1
    else:
        # All failed - this is expected for invalid vectors
        print(f"PASS {name} (all rejected)")
        pass_count += 1

sdk_list = ", ".join(sdks.keys())
print("")
print(f"=== Results ({sdk_list}) ===")
print(f"Passed: {pass_count}")
print(f"Failed: {fail_count}")

if fail_count > 0:
    exit(1)

print("")
print("All conformance tests passed!")
COMPARE_SCRIPT
