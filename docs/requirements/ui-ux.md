[← Metaphor](./metaphor.md) | [Index](./index.md) | [Visual →](./visual.md)

# UI/UX Design

画面構成・操作・視覚フィードバックの定義。

---

## Layout

```
┌────────────────────────────────────┬──────┐
│ [/ > Documents > Projects]         │  ≡  │
│           (overlay)                ├──────┤
│                                    │      │
│           3D Universe              │ Side │
│                                    │ bar  │
│         tooltip [file.txt]         │      │
│                                    │      │
└────────────────────────────────────┴──────┘
```

| 要素 | 仕様 |
|------|------|
| 3Dビュー | メイン領域。宇宙空間を表示 |
| サイドバー | 右側、トグル式。機能・設定を配置 |
| パンくず | 3Dビュー内オーバーレイ（半透明） |
| ツールチップ | 選択天体の詳細情報を表示 |

---

## Startup

- 空の宇宙を表示
- 「フォルダを開く」ボタンを中央に配置
- フォルダ選択後、宇宙が生成される

---

## Visual Feedback

| 状態 | 表現 |
|------|------|
| ホバー | ハイライト + 名前表示 |
| 選択 | 枠線 or 発光 |
| ドリルダウン | ズームアニメーション |

---

## Theme

- OS追従（macOSのダーク/ライト設定に従う）

---

## Keyboard (MVP)

| キー | 動作 |
|------|------|
| Esc | 選択解除 |
| Space | ビューをリセット |

---

## Scope (MVP)

- 閲覧のみ（ファイル編集・削除は後回し）

---

## See Also

- [Core Metaphor](./metaphor.md) - 操作マッピングの定義
- [Visual Encoding](./visual.md) - 視覚表現ルール
- [Tech Stack](./tech.md) - UI実装技術（egui）
