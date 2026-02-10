# Starfield Implementation Status

背景星空の実装方針と進捗。

---

## Approach Comparison

| # | Approach | Draw Call | Code | Natural | Deps | Status |
|---|----------|-----------|------|---------|------|--------|
| A | **Single Mesh（頂点色）** | 1 | Small | High | None | **Implementing** |
| B | Skybox cubemap | 0 | Medium | Best | None (Bevy built-in) | Pending |

---

## A: Single Mesh (Current)

200個の星を1つのメッシュにまとめ、頂点色でバリエーションを表現。

| Item | Value |
|------|-------|
| Entity count | 1 |
| Draw calls | 1 |
| Distribution | Seeded random (natural) |
| Color variation | Per-vertex color |
| Interaction | None (no Clickable) |

### Pros
- 200 draw call → 1 draw call
- 追加依存なし
- コード簡潔

### Cons
- メッシュ生成コードが必要
- Skyboxほどの視覚品質ではない

---

## B: Skybox Cubemap (Future)

Bevy内蔵の `Skybox` コンポーネントでcubemapテクスチャを背景に表示。

| Item | Value |
|------|-------|
| Entity count | 0 (camera component) |
| Draw calls | 0 |
| Bevy component | `Skybox { image, brightness, rotation }` |
| Format | KTX2 (recommended) or PNG (6:1 strip) |

### Pros
- 最高パフォーマンス（0 draw call）
- 星雲等も表現可能
- 業界標準

### Cons
- Cubemap 6面テクスチャの生成/調達が必要
- [Bevy Issue #19125](https://github.com/bevyengine/bevy/issues/19125): cubemapインポートの困難さ
- 手続き生成は面投影計算が複雑

### Implementation Notes
```rust
// Bevy 0.15 Skybox usage
commands.spawn((
    Camera3d::default(),
    Skybox {
        image: cubemap_handle,
        brightness: 1000.0,
        rotation: Quat::IDENTITY,
    },
));
```

---

## Crate Survey (2026-02-10)

| Crate | Bevy 0.15 | Notes |
|-------|:---------:|-------|
| `bevy_starfield` 0.1.1 | No (0.10) | Yale Star Catalog, 2+ years stale |
| `bevy_skybox` 0.6.0 | Yes | Net image → cubemap helper only |
| `bevy_atmosphere` 0.13.0 | Yes | Atmospheric scattering, not space |

No Bevy 0.15 compatible starfield crate exists.
