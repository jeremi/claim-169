/**
 * Tests that verify documentation code examples work correctly.
 *
 * Each test corresponds to a code example in the docs.
 * If a test here fails, the corresponding docs page has a broken example.
 */

import { describe, it, expect } from "vitest";
import { Decoder, hexToBytes } from "../src/index";

describe("Doc Examples", () => {
  // Getting Started: Decode Your First QR Code (TypeScript tab)
  // Docs: docs/en/getting-started.md, docs/fr/getting-started.md
  it("getting-started decode example", () => {
    const qrData =
      "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4";

    const publicKey = hexToBytes(
      "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    );

    const result = new Decoder(qrData)
      .verifyWithEd25519(publicKey)
      .decode();

    expect(result.claim169.id).toBe("ID-SIGNED-001");
    expect(result.claim169.fullName).toBe("Signed Test Person");
    expect(result.cwtMeta.issuer).toBe("https://mosip.example.org");
    expect(result.verificationStatus).toBe("verified");
  });

  // Landing page: Quick Example (TypeScript tab)
  // Docs: docs/en/index.md, docs/fr/index.md
  it("landing page quick example API shape", () => {
    const qrData =
      "6BF590B20FFWJWG.FKJ05H7B0XKA8FA9DIWENPEJ/5P$DPQE88EB$CBECP9ERZC04E21DDF3/E96007F3ORAO001KL580 B9%W5*B9C+9%R8646%86HKESED1/DRTC5UA QE$345$CVQEX.DX88WBK0NG8PB4 O/38TL6XDALLKLPQATHO.3ZPJMUAVQFSB1:+B*21V FWMC6SU439YU774475LJ2U5T02$VBSIMLQ3:6J.E1-1STM$4";

    const publicKey = hexToBytes(
      "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    );

    const result = new Decoder(qrData)
      .verifyWithEd25519(publicKey)
      .decode();

    // Verify the attribute names used in docs exist
    expect(result.claim169.fullName).toBeDefined();
    expect(result.claim169.id).toBeDefined();
    expect(result.verificationStatus).toBeDefined();
  });
});
