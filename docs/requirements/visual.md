[← UI/UX](./ui-ux.md) | [Index](./index.md) | [Size Calculation →](./size-calculation.md)

# Visual Encoding

天体の視覚表現ルール。

---

## Size（サイズ → 容量）

| 対象 | スケール | 範囲 |
|------|---------|------|
| ディレクトリ | log10 | 0.5 〜 2.0 |
| ファイル | log10 | 0.3 〜 1.8 |
| 恒星（現在フォルダ） | 固定 | 2.5 |

---

## Color（色 → ファイル種別）

| 種別 | 色 | Hex |
|------|-----|-----|
| Code | Cyan | `#61dafb` |
| Image | Orange | `#f59e0b` |
| Video | Red | `#ef4444` |
| Document | Blue | `#3b82f6` |
| Data | Teal | `#06b6d4` |
| Archive | Gray | `#6b7280` |
| Directory | White | `#ffffff` |

---

## Brightness（明るさ → 更新日時）

| 最終更新 | 明るさ |
|---------|--------|
| 24時間以内 | 100% |
| 1週間以内 | 85% |
| 1ヶ月以内 | 70% |
| 3ヶ月以内 | 55% |
| 1年以内 | 40% |
| 1年以上 | 25% |

---

## Background（背景星空）

| 項目 | 値 |
|------|-----|
| 星の数 | 300個 |
| 配置半径 | 150.0（カメラ最大zoom 100超） |
| 配置方式 | ランダム球面分布（deterministic hash） |
| 実装 | Single Mesh（1 entity, 1 draw call, per-vertex color） |
| サイズ | 通常 0.15〜0.40、明るい星 0.50〜1.00（~10%） |
| 色 | 白青系60%、暖白25%、淡黄15% |
| マテリアル | Unlit, double_sided（光源不要） |
| ClearColor | `srgb_u8(3, 3, 8)`（極暗紺色） |
| インタラクション | なし（Clickable非付与） |
| ライフサイクル | Empty進入時に生成、永続（cleanup対象外） |

---

## Display Limits（表示制約）

| 項目 | 値 |
|------|-----|
| 最大表示数 | 20個（21個目以降はアステロイドベルト） |
| 表示深度 | 1（恒星 + 惑星のみ、孫はリング抽象化） |

---

## See Also

- [Core Metaphor](./metaphor.md) - エンティティ定義
- [UI/UX Design](./ui-ux.md) - 視覚フィードバック
