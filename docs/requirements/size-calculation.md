[← Visual](./visual.md) | [Index](./index.md)

# Size Calculation

ディレクトリ/ファイルサイズ計算の要件定義。

---

## Overview

| 項目 | 決定内容 |
|------|----------|
| **計算タイミング** | フォルダ表示時に自動（バックグラウンド） |
| **計算対象** | 全ての子ディレクトリ |
| **高速化方針** | 最高性能を優先 |
| **アーキテクチャ** | OS依存部分をモジュール化、ビルド時に最適構成 |

---

## Architecture: Platform Abstraction

```
┌─────────────────────────────────────────────────┐
│              SizeCalculator Trait               │
│  fn calculate(&self, path: &Path) -> u64        │
│  fn calculate_async(&self, path: &Path) -> Rx   │
└─────────────────────────────────────────────────┘
            │                    │
            ▼                    ▼
┌─────────────────┐    ┌─────────────────┐
│ SpotlightCalc   │    │ JwalkCalc       │
│ (macOS)         │    │ (Cross-platform)│
│ #[cfg(macos)]   │    │ #[cfg(not)]     │
└─────────────────┘    └─────────────────┘
```

### ビルド時選択

```toml
# Cargo.toml
[features]
default = ["spotlight"]  # macOSデフォルト
spotlight = []           # macOS Spotlight API
jwalk = ["dep:jwalk"]    # クロスプラットフォーム
```

---

## Platform: macOS (Primary)

| 項目 | 値 |
|------|-----|
| API | Spotlight (mdfind/mdls) |
| 期待性能 | 10x+ 高速 |
| 制限 | インデックス済みファイルのみ |

### Spotlight API

```bash
# ディレクトリ内の全ファイルサイズ合計
mdfind -onlyin /path "kMDItemFSSize > 0" -attr kMDItemFSSize
```

---

## Platform: Windows/Linux (Fallback)

| 項目 | 値 |
|------|-----|
| 実装 | jwalk + rayon 並列走査 |
| 期待性能 | 4x 高速（walkdir比） |
| 依存 | `jwalk = "0.8"` |

---

## UI Behavior

### 表示フロー

```mermaid
flowchart LR
    A["フォルダ選択"] --> B["天体を最小サイズで即座に表示"]
    B --> C["パルスアニメーション開始"]
    C --> D["バックグラウンド計算"]
    D --> E["計算完了"]
    E --> F["サイズ反映 + アニメーション停止"]
```

### 計算中の視覚表現

| 状態 | 表現 |
|------|------|
| 計算中 | **パルス（明滅）アニメーション** |
| 完了 | アニメーション停止、正しいサイズに更新 |
| エラー | アニメーション停止、最小サイズのまま |

---

## Data Flow

```
1. フォルダ選択
   ↓
2. read_directory() → FileEntry { size_bytes: 0 } でディレクトリ生成
   ↓
3. spawn_celestials() → 最小サイズで天体表示 + PulseAnimation追加
   ↓
4. IoTaskPool::spawn() → SizeCalculator::calculate_async()
   ↓
5. async_channel で結果を送信
   ↓
6. update_celestial_sizes() → エンティティ更新 + アニメーション停止
```

---

## Component Design

### 新規コンポーネント

| コンポーネント | 用途 |
|---------------|------|
| `PendingSizeCalculation` | 計算待ちマーカー |
| `PulseAnimation` | 明滅アニメーション状態 |

### 新規リソース

| リソース | 用途 |
|---------|------|
| `SizeCalculationChannel` | バックグラウンド計算結果の受信 |

---

## Performance Targets

| プラットフォーム | フォルダ規模 | 目標時間 |
|-----------------|-------------|----------|
| macOS (Spotlight) | 100,000 files | < 1秒 |
| Others (jwalk) | 100,000 files | < 5秒 |

---

## Implementation Priority

| 優先度 | 内容 |
|--------|------|
| **P0** | macOS Spotlight実装 |
| **P0** | パルスアニメーション |
| **P0** | IoTaskPoolバックグラウンド計算 |
| **P1** | jwalkフォールバック実装 |
| **P2** | キャッシュ追加 |

---

## See Also

- [Visual Encoding](./visual.md) - サイズ → 天体サイズのマッピング
- [Size Calculation Research](../reference/size-calculation-research.md) - 技術調査結果
