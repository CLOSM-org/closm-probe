'use client';

import { useState, useMemo } from 'react';
import { UniverseCanvas } from './universe/UniverseCanvas';
import {
  FileNode,
  PositionedItem,
  typeColors,
  formatSize,
  formatTimeAgo,
} from './universe/types';

// Sample file system data (will be replaced with Google Drive API)
const sampleFileSystem: FileNode = {
  name: 'root',
  type: 'directory',
  size: 0,
  children: [
    {
      name: 'プロジェクト',
      type: 'directory',
      size: 0,
      children: [
        {
          name: 'WebApp',
          type: 'directory',
          size: 0,
          children: [
            { name: 'index.html', type: 'file', fileType: 'code', size: 15000, lastModified: Date.now() - 86400000 },
            { name: 'style.css', type: 'file', fileType: 'code', size: 8000, lastModified: Date.now() - 172800000 },
            { name: 'app.js', type: 'file', fileType: 'code', size: 45000, lastModified: Date.now() - 3600000 },
            { name: 'bundle.min.js', type: 'file', fileType: 'code', size: 250000, lastModified: Date.now() - 604800000 },
          ]
        },
        {
          name: 'デザイン',
          type: 'directory',
          size: 0,
          children: [
            { name: 'mockup_v1.fig', type: 'file', fileType: 'design', size: 5200000, lastModified: Date.now() - 259200000 },
            { name: 'mockup_v2.fig', type: 'file', fileType: 'design', size: 8100000, lastModified: Date.now() - 86400000 },
            { name: 'icons.svg', type: 'file', fileType: 'image', size: 120000, lastModified: Date.now() - 432000000 },
            { name: 'hero_image.png', type: 'file', fileType: 'image', size: 3500000, lastModified: Date.now() - 518400000 },
          ]
        },
        {
          name: 'ドキュメント',
          type: 'directory',
          size: 0,
          children: [
            { name: '企画書.pdf', type: 'file', fileType: 'pdf', size: 2800000, lastModified: Date.now() - 1209600000 },
            { name: 'API仕様.md', type: 'file', fileType: 'doc', size: 35000, lastModified: Date.now() - 172800000 },
            { name: '議事録.txt', type: 'file', fileType: 'text', size: 12000, lastModified: Date.now() - 43200000 },
          ]
        }
      ]
    },
    {
      name: 'メディア',
      type: 'directory',
      size: 0,
      children: [
        {
          name: '写真',
          type: 'directory',
          size: 0,
          children: [
            { name: 'vacation_001.jpg', type: 'file', fileType: 'image', size: 4200000, lastModified: Date.now() - 2592000000 },
            { name: 'vacation_002.jpg', type: 'file', fileType: 'image', size: 3800000, lastModified: Date.now() - 2592000000 },
            { name: 'vacation_003.jpg', type: 'file', fileType: 'image', size: 4500000, lastModified: Date.now() - 2592000000 },
            { name: 'screenshot.png', type: 'file', fileType: 'image', size: 850000, lastModified: Date.now() - 604800000 },
          ]
        },
        {
          name: '動画',
          type: 'directory',
          size: 0,
          children: [
            { name: 'demo_recording.mp4', type: 'file', fileType: 'video', size: 125000000, lastModified: Date.now() - 1814400000 },
            { name: 'tutorial.mp4', type: 'file', fileType: 'video', size: 89000000, lastModified: Date.now() - 3628800000 },
          ]
        }
      ]
    },
    {
      name: 'バックアップ',
      type: 'directory',
      size: 0,
      children: [
        { name: 'db_backup_2024.sql', type: 'file', fileType: 'data', size: 45000000, lastModified: Date.now() - 604800000 },
        { name: 'config_backup.zip', type: 'file', fileType: 'archive', size: 12000000, lastModified: Date.now() - 1209600000 },
      ]
    }
  ]
};

// Calculate directory sizes recursively
function calculateSizes(node: FileNode): number {
  if (node.type === 'file') {
    return node.size;
  }
  let total = 0;
  if (node.children) {
    node.children.forEach(child => {
      total += calculateSizes(child);
    });
  }
  node.size = total;
  return total;
}

// Flatten tree with 3D positions
function flattenWithPositions(
  node: FileNode,
  depth = 0,
  angle = 0,
  parentPos = { x: 0, y: 0, z: 0 },
  path = ''
): PositionedItem[] {
  const items: PositionedItem[] = [];
  const currentPath = path ? `${path}/${node.name}` : node.name;

  if (depth === 0) {
    items.push({
      ...node,
      path: currentPath,
      depth,
      x: 0,
      y: 0,
      z: 0,
      orbitRadius: 0
    });

    if (node.children) {
      const childCount = node.children.length;
      node.children.forEach((child, i) => {
        const childAngle = (i / childCount) * Math.PI * 2;
        items.push(...flattenWithPositions(child, depth + 1, childAngle, { x: 0, y: 0, z: 0 }, currentPath));
      });
    }
    return items;
  }

  const baseRadius = 2 + depth * 1.8;
  const orbitRadius = baseRadius;

  const x = parentPos.x + Math.cos(angle) * orbitRadius;
  const z = parentPos.z + Math.sin(angle) * orbitRadius;
  const y = parentPos.y + (depth - 1) * 0.5;

  items.push({
    ...node,
    path: currentPath,
    depth,
    x,
    y,
    z,
    angle,
    orbitRadius,
    parentPos
  });

  if (node.children && node.children.length > 0) {
    const childCount = node.children.length;
    node.children.forEach((child, i) => {
      const childAngle = angle + ((i - (childCount - 1) / 2) * 0.5);
      items.push(...flattenWithPositions(child, depth + 1, childAngle, { x, y, z }, currentPath));
    });
  }

  return items;
}

// Initialize sizes
calculateSizes(sampleFileSystem);

// Helper: shade color
function shadeColor(color: string, percent: number): string {
  const num = parseInt(color.replace('#', ''), 16);
  const amt = Math.round(2.55 * percent);
  const R = Math.min(255, Math.max(0, (num >> 16) + amt));
  const G = Math.min(255, Math.max(0, (num >> 8 & 0x00FF) + amt));
  const B = Math.min(255, Math.max(0, (num & 0x0000FF) + amt));
  return '#' + (0x1000000 + R * 0x10000 + G * 0x100 + B).toString(16).slice(1);
}

export default function PhysicalStorageUniverse() {
  const [selectedItem, setSelectedItem] = useState<PositionedItem | null>(null);
  const [hoveredItem, setHoveredItem] = useState<PositionedItem | null>(null);

  const items = useMemo(() => flattenWithPositions(sampleFileSystem), []);
  const totalSize = sampleFileSystem.size;

  const breadcrumbs = selectedItem?.path.split('/').filter(Boolean) || [];

  return (
    <div style={{
      minHeight: '100vh',
      background: 'linear-gradient(180deg, #0a0a1a 0%, #1a1a2e 100%)',
      color: 'white',
      padding: '20px',
      fontFamily: 'system-ui, sans-serif'
    }}>
      <div style={{ maxWidth: '1200px', margin: '0 auto' }}>
        {/* Header */}
        <div style={{ marginBottom: '20px' }}>
          <h1 style={{
            fontSize: '32px',
            fontWeight: 'bold',
            background: 'linear-gradient(90deg, #a855f7, #3b82f6, #06b6d4)',
            WebkitBackgroundClip: 'text',
            WebkitTextFillColor: 'transparent',
            marginBottom: '8px'
          }}>
            Storage Universe
          </h1>
          <p style={{ color: '#888', fontSize: '14px' }}>
            ストレージを宇宙空間として可視化 - ドラッグで回転 - スクロールでズーム - ダブルクリックでフォーカス
          </p>
        </div>

        {/* Stats */}
        <div style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(auto-fit, minmax(150px, 1fr))',
          gap: '12px',
          marginBottom: '20px'
        }}>
          <div style={{
            background: 'rgba(168, 85, 247, 0.1)',
            border: '1px solid rgba(168, 85, 247, 0.3)',
            borderRadius: '12px',
            padding: '16px'
          }}>
            <div style={{ fontSize: '24px', fontWeight: 'bold' }}>{formatSize(totalSize)}</div>
            <div style={{ fontSize: '12px', color: '#888' }}>総容量</div>
          </div>
          <div style={{
            background: 'rgba(59, 130, 246, 0.1)',
            border: '1px solid rgba(59, 130, 246, 0.3)',
            borderRadius: '12px',
            padding: '16px'
          }}>
            <div style={{ fontSize: '24px', fontWeight: 'bold' }}>{items.filter(i => i.type === 'directory').length}</div>
            <div style={{ fontSize: '12px', color: '#888' }}>フォルダ数</div>
          </div>
          <div style={{
            background: 'rgba(6, 182, 212, 0.1)',
            border: '1px solid rgba(6, 182, 212, 0.3)',
            borderRadius: '12px',
            padding: '16px'
          }}>
            <div style={{ fontSize: '24px', fontWeight: 'bold' }}>{items.filter(i => i.type === 'file').length}</div>
            <div style={{ fontSize: '12px', color: '#888' }}>ファイル数</div>
          </div>
        </div>

        {/* Main view */}
        <div style={{ display: 'flex', gap: '20px', flexWrap: 'wrap' }}>
          {/* 3D Canvas */}
          <div style={{ flex: '1 1 700px' }}>
            <UniverseCanvas
              items={items}
              selectedItem={selectedItem}
              hoveredItem={hoveredItem}
              onSelect={setSelectedItem}
              onHover={setHoveredItem}
            />
          </div>

          {/* Detail Panel */}
          <div style={{ flex: '1 1 280px', minWidth: '280px' }}>
            {selectedItem ? (
              <div style={{
                background: 'rgba(255,255,255,0.03)',
                borderRadius: '16px',
                padding: '20px',
                border: '1px solid rgba(255,255,255,0.1)'
              }}>
                {/* Breadcrumbs */}
                <div style={{
                  fontSize: '11px',
                  color: '#666',
                  marginBottom: '12px',
                  display: 'flex',
                  flexWrap: 'wrap',
                  gap: '4px'
                }}>
                  {breadcrumbs.map((crumb, i) => (
                    <span key={i}>
                      {i > 0 && <span style={{ margin: '0 4px' }}>/</span>}
                      <span style={{ color: i === breadcrumbs.length - 1 ? '#a855f7' : '#888' }}>
                        {crumb}
                      </span>
                    </span>
                  ))}
                </div>

                {/* Icon and name */}
                <div style={{ display: 'flex', alignItems: 'center', gap: '12px', marginBottom: '16px' }}>
                  <div style={{
                    width: '56px',
                    height: '56px',
                    borderRadius: '14px',
                    background: `linear-gradient(135deg, ${selectedItem.type === 'directory' ? typeColors.directory : typeColors[selectedItem.fileType || ''] || '#888'}, ${shadeColor(selectedItem.type === 'directory' ? typeColors.directory : typeColors[selectedItem.fileType || ''] || '#888', -30)})`,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    fontSize: '28px'
                  }}>
                    {selectedItem.type === 'directory' ? '📁' :
                     selectedItem.fileType === 'code' ? '💻' :
                     selectedItem.fileType === 'design' ? '🎨' :
                     selectedItem.fileType === 'image' ? '🖼️' :
                     selectedItem.fileType === 'video' ? '🎬' :
                     selectedItem.fileType === 'pdf' ? '📄' :
                     selectedItem.fileType === 'doc' ? '📝' :
                     selectedItem.fileType === 'data' ? '🗄️' :
                     selectedItem.fileType === 'archive' ? '📦' : '📄'}
                  </div>
                  <div>
                    <h3 style={{ fontSize: '16px', fontWeight: 'bold', margin: 0, wordBreak: 'break-all' }}>
                      {selectedItem.name}
                    </h3>
                    <div style={{ fontSize: '12px', color: '#888', marginTop: '4px' }}>
                      {selectedItem.type === 'directory' ? 'フォルダ' : selectedItem.fileType}
                    </div>
                  </div>
                </div>

                {/* Details */}
                <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
                  <div style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    padding: '10px',
                    background: 'rgba(255,255,255,0.03)',
                    borderRadius: '8px'
                  }}>
                    <span style={{ color: '#888', fontSize: '13px' }}>サイズ</span>
                    <span style={{ fontWeight: 'bold', fontSize: '13px' }}>{formatSize(selectedItem.size)}</span>
                  </div>

                  {selectedItem.type === 'directory' && (
                    <div style={{
                      display: 'flex',
                      justifyContent: 'space-between',
                      padding: '10px',
                      background: 'rgba(255,255,255,0.03)',
                      borderRadius: '8px'
                    }}>
                      <span style={{ color: '#888', fontSize: '13px' }}>アイテム数</span>
                      <span style={{ fontWeight: 'bold', fontSize: '13px' }}>
                        {selectedItem.children?.length || 0}
                      </span>
                    </div>
                  )}

                  {selectedItem.lastModified && (
                    <div style={{
                      display: 'flex',
                      justifyContent: 'space-between',
                      padding: '10px',
                      background: 'rgba(255,255,255,0.03)',
                      borderRadius: '8px'
                    }}>
                      <span style={{ color: '#888', fontSize: '13px' }}>更新</span>
                      <span style={{ fontWeight: 'bold', fontSize: '13px' }}>
                        {formatTimeAgo(selectedItem.lastModified)}
                      </span>
                    </div>
                  )}

                  <div style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    padding: '10px',
                    background: 'rgba(255,255,255,0.03)',
                    borderRadius: '8px'
                  }}>
                    <span style={{ color: '#888', fontSize: '13px' }}>階層</span>
                    <span style={{ fontWeight: 'bold', fontSize: '13px' }}>
                      {selectedItem.depth === 0 ? 'ルート' : `レベル ${selectedItem.depth}`}
                    </span>
                  </div>
                </div>

                {/* Progress bar */}
                <div style={{ marginTop: '16px' }}>
                  <div style={{ fontSize: '12px', color: '#888', marginBottom: '6px' }}>
                    全体に占める割合
                  </div>
                  <div style={{
                    height: '8px',
                    background: 'rgba(255,255,255,0.1)',
                    borderRadius: '4px',
                    overflow: 'hidden'
                  }}>
                    <div style={{
                      height: '100%',
                      width: `${Math.max(1, (selectedItem.size / totalSize) * 100)}%`,
                      background: `linear-gradient(90deg, ${selectedItem.type === 'directory' ? typeColors.directory : typeColors[selectedItem.fileType || ''] || '#888'}, ${shadeColor(selectedItem.type === 'directory' ? typeColors.directory : typeColors[selectedItem.fileType || ''] || '#888', 20)})`,
                      borderRadius: '4px'
                    }} />
                  </div>
                  <div style={{ fontSize: '11px', color: '#666', marginTop: '4px', textAlign: 'right' }}>
                    {((selectedItem.size / totalSize) * 100).toFixed(1)}%
                  </div>
                </div>
              </div>
            ) : (
              <div style={{
                background: 'rgba(255,255,255,0.03)',
                borderRadius: '16px',
                padding: '40px 20px',
                textAlign: 'center',
                border: '1px solid rgba(255,255,255,0.1)'
              }}>
                <div style={{ fontSize: '48px', marginBottom: '12px' }}>🎯</div>
                <div style={{ color: '#888', fontSize: '14px' }}>
                  ノードをクリックして<br/>詳細を表示
                </div>
              </div>
            )}

            {/* Legend */}
            <div style={{
              marginTop: '16px',
              background: 'rgba(255,255,255,0.03)',
              borderRadius: '12px',
              padding: '16px',
              border: '1px solid rgba(255,255,255,0.1)'
            }}>
              <div style={{ fontSize: '12px', color: '#888', marginBottom: '10px' }}>ファイルタイプ</div>
              <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px' }}>
                {Object.entries(typeColors).map(([type, color]) => (
                  <div key={type} style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '4px',
                    fontSize: '11px'
                  }}>
                    <div style={{
                      width: '10px',
                      height: '10px',
                      borderRadius: '50%',
                      background: color
                    }} />
                    <span style={{ color: '#aaa' }}>{type}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>

        {/* Instructions */}
        <div style={{
          marginTop: '20px',
          display: 'flex',
          justifyContent: 'center',
          gap: '24px',
          fontSize: '12px',
          color: '#666'
        }}>
          <span>ドラッグ: 回転</span>
          <span>スクロール: ズーム</span>
          <span>クリック: 選択</span>
          <span>ダブルクリック: フォーカス</span>
        </div>
      </div>
    </div>
  );
}
