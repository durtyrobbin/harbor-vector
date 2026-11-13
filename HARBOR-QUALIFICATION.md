# Harbor Vector qualification

Companion source fork for Harbor Studio, 2026-11-13.

The vector engine and UI compiled as part of the successful optimized native Harbor Studio build. The source retains the editable shared-canvas adapter and original attribution/history.

This repository is currently an engine/UI companion. It is not a separately qualified Harbor Vector standalone executable, installer or web distribution. Legacy packaging retains upstream identifiers and must not be used as a Harbor installer without separate work.

Scoped checks passed: vector UI formatting, WASM library compilation, and architecture layers (27 crates, no violations). Studio's five active host regression tests passed, including real offscreen vector input and native-content round trips; both editor images were visually inspected. A complete standalone Vector CI suite was not run.
