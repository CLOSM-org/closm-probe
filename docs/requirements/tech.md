[← Size Calculation](./size-calculation.md) | [Index](./index.md)

# Tech Stack

技術選定と制約。

---

## Stack

| 項目 | 選定 |
|------|------|
| 言語 | Rust (Edition 2024) |
| エンジン | Bevy 0.15 |
| UI | egui (bevy_egui) |
| パーティクル | bevy_hanabi |

---

## Target Platform

| プラットフォーム | 優先度 |
|-----------------|--------|
| macOS | P0（最優先） |
| Windows | P1 |
| Linux | P1 |

---

## Constraints

- ネイティブアプリ（Webは将来検討）
- ローカルファイルシステムのみ（クラウドは将来検討）

---

## See Also

- [Visual Encoding](./visual.md) - 視覚表現ルール
- [UI/UX Design](./ui-ux.md) - egui での UI 実装仕様
