'use client';

import { useState, useMemo, useCallback, useRef } from 'react';
import { UniverseCanvas } from './universe/UniverseCanvas';
import { CameraControllerRef } from './universe/controls/CameraController';
import {
  FileNode,
  PositionedItem,
  AsteroidBelt,
  typeColors,
  formatSize,
  formatTimeAgo,
  calculateEqualSpacedAngle,
  calculateOrbitRadiusByOrder,
  sortByLastModified,
  sortByCreatedAt,
  classifyChildren,
} from './universe/types';

// Time constants for sample data
const DAY = 86400000;
const WEEK = DAY * 7;
const MONTH = DAY * 30;

// Sample file system data (will be replaced with Google Drive API)
const sampleFileSystem: FileNode = {
  name: 'root',
  type: 'directory',
  size: 0,
  createdAt: Date.now() - MONTH * 12,
  children: [
    {
      name: 'プロジェクト',
      type: 'directory',
      size: 0,
      createdAt: Date.now() - MONTH * 6,
      lastModified: Date.now() - DAY,
      children: [
        {
          name: 'WebApp',
          type: 'directory',
          size: 0,
          createdAt: Date.now() - MONTH * 3,
          lastModified: Date.now() - DAY,
          children: [
            { name: 'index.html', type: 'file', fileType: 'code', size: 15000, createdAt: Date.now() - MONTH * 3, lastModified: Date.now() - DAY },
            { name: 'style.css', type: 'file', fileType: 'code', size: 8000, createdAt: Date.now() - MONTH * 3, lastModified: Date.now() - DAY * 2 },
            { name: 'app.js', type: 'file', fileType: 'code', size: 45000, createdAt: Date.now() - MONTH * 2, lastModified: Date.now() - 3600000 },
            { name: 'bundle.min.js', type: 'file', fileType: 'code', size: 250000, createdAt: Date.now() - MONTH, lastModified: Date.now() - WEEK },
          ]
        },
        {
          name: 'デザイン',
          type: 'directory',
          size: 0,
          createdAt: Date.now() - MONTH * 6,
          lastModified: Date.now() - DAY,
          children: [
            { name: 'mockup_v1.fig', type: 'file', fileType: 'design', size: 5200000, createdAt: Date.now() - MONTH * 6, lastModified: Date.now() - MONTH * 2 },
            { name: 'mockup_v2.fig', type: 'file', fileType: 'design', size: 8100000, createdAt: Date.now() - MONTH, lastModified: Date.now() - DAY },
            { name: 'icons.svg', type: 'file', fileType: 'image', size: 120000, createdAt: Date.now() - MONTH * 4, lastModified: Date.now() - MONTH },
            { name: 'hero_image.png', type: 'file', fileType: 'image', size: 3500000, createdAt: Date.now() - MONTH * 5, lastModified: Date.now() - MONTH * 2 },
          ]
        },
        {
          name: 'ドキュメント',
          type: 'directory',
          size: 0,
          createdAt: Date.now() - MONTH * 6,
          lastModified: Date.now() - DAY,
          children: [
            { name: '企画書.pdf', type: 'file', fileType: 'pdf', size: 2800000, createdAt: Date.now() - MONTH * 6, lastModified: Date.now() - MONTH * 2 },
            { name: 'API仕様.md', type: 'file', fileType: 'doc', size: 35000, createdAt: Date.now() - MONTH * 2, lastModified: Date.now() - DAY * 2 },
            { name: '議事録.txt', type: 'file', fileType: 'text', size: 12000, createdAt: Date.now() - WEEK, lastModified: Date.now() - DAY / 2 },
          ]
        }
      ]
    },
    {
      name: 'メディア',
      type: 'directory',
      size: 0,
      createdAt: Date.now() - MONTH * 10,
      lastModified: Date.now() - WEEK,
      children: [
        {
          name: '写真',
          type: 'directory',
          size: 0,
          createdAt: Date.now() - MONTH * 10,
          lastModified: Date.now() - MONTH,
          children: [
            { name: 'vacation_001.jpg', type: 'file', fileType: 'image', size: 4200000, createdAt: Date.now() - MONTH * 10, lastModified: Date.now() - MONTH * 10 },
            { name: 'vacation_002.jpg', type: 'file', fileType: 'image', size: 3800000, createdAt: Date.now() - MONTH * 10, lastModified: Date.now() - MONTH * 10 },
            { name: 'vacation_003.jpg', type: 'file', fileType: 'image', size: 4500000, createdAt: Date.now() - MONTH * 10, lastModified: Date.now() - MONTH * 10 },
            { name: 'screenshot.png', type: 'file', fileType: 'image', size: 850000, createdAt: Date.now() - WEEK, lastModified: Date.now() - WEEK },
          ]
        },
        {
          name: '動画',
          type: 'directory',
          size: 0,
          createdAt: Date.now() - MONTH * 8,
          lastModified: Date.now() - MONTH * 2,
          children: [
            { name: 'demo_recording.mp4', type: 'file', fileType: 'video', size: 125000000, createdAt: Date.now() - MONTH * 2, lastModified: Date.now() - MONTH * 2 },
            { name: 'tutorial.mp4', type: 'file', fileType: 'video', size: 89000000, createdAt: Date.now() - MONTH * 8, lastModified: Date.now() - MONTH * 6 },
          ]
        }
      ]
    },
    {
      name: 'バックアップ',
      type: 'directory',
      size: 0,
      createdAt: Date.now() - MONTH * 12,
      lastModified: Date.now() - WEEK,
      children: [
        { name: 'db_backup_2024.sql', type: 'file', fileType: 'data', size: 45000000, createdAt: Date.now() - WEEK, lastModified: Date.now() - WEEK },
        { name: 'config_backup.zip', type: 'file', fileType: 'archive', size: 12000000, createdAt: Date.now() - MONTH * 12, lastModified: Date.now() - MONTH * 2 },
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

// Find node by path
function findNodeByPath(root: FileNode, path: string): FileNode | null {
  const parts = path.split('/').filter(Boolean);
  let current: FileNode | null = root;

  for (let i = 1; i < parts.length; i++) { // Skip root name
    if (!current || !current.children) return null;
    const found: FileNode | undefined = current.children.find(c => c.name === parts[i]);
    if (!found) return null;
    current = found;
  }

  return current;
}

// Result type for flatten function
interface FlattenResult {
  items: PositionedItem[];
  asteroidBelts: AsteroidBelt[];
}

// Flatten tree with 3D positions - Solar System style with sorted equal-spacing
// θ (angle) = equal spacing, sorted by last modified (newest at 12 o'clock)
// r (radius) = sorted by creation date (newer = inner orbit)
// Small files are collected into asteroid belts
function flattenWithPositions(
  node: FileNode,
  depth = 0,
  sortedAngleIndex = 0,
  sortedRadiusIndex = 0,
  siblingCount = 1,
  parentPos = { x: 0, y: 0, z: 0 },
  path = '',
  maxDepth = 2
): FlattenResult {
  const items: PositionedItem[] = [];
  const asteroidBelts: AsteroidBelt[] = [];
  const currentPath = path ? `${path}/${node.name}` : node.name;

  // Stop at maxDepth
  if (depth > maxDepth) {
    return { items, asteroidBelts };
  }

  if (depth === 0) {
    // Sun (center)
    items.push({
      ...node,
      path: currentPath,
      depth,
      x: 0,
      y: 0,
      z: 0,
      orbitRadius: 0
    });

    if (node.children && node.children.length > 0) {
      // Classify children into significant items and asteroids
      const { significantItems, asteroids } = classifyChildren(node.children);

      // Create asteroid belt if there are asteroids
      if (asteroids.length > 0) {
        asteroidBelts.push({
          id: `belt-${currentPath}`,
          parentPath: currentPath,
          parentPos: { x: 0, y: 0, z: 0 },
          orbitRadius: 40, // Outer orbit for asteroid belt
          files: asteroids,
          count: asteroids.length,
          totalSize: asteroids.reduce((sum, a) => sum + a.size, 0),
        });
      }

      // Sort significant items for placement
      const sortedByModified = sortByLastModified(significantItems);
      const sortedByCreation = sortByCreatedAt(significantItems);

      sortedByModified.forEach((child, angleIndex) => {
        const radiusIndex = sortedByCreation.findIndex(c => c.name === child.name);
        const result = flattenWithPositions(
          child,
          depth + 1,
          angleIndex,
          radiusIndex,
          sortedByModified.length,
          { x: 0, y: 0, z: 0 },
          currentPath,
          maxDepth
        );
        items.push(...result.items);
        asteroidBelts.push(...result.asteroidBelts);
      });
    }
    return { items, asteroidBelts };
  }

  // Calculate orbital radius based on creation order (newer = inner)
  const baseRadius = depth === 1 ? 25 : 8;
  const orbitRadius = calculateOrbitRadiusByOrder(sortedRadiusIndex, siblingCount, baseRadius);

  // Calculate angle based on sorted position (equal spacing)
  const angle = calculateEqualSpacedAngle(sortedAngleIndex, siblingCount);

  // Convert polar to cartesian (flat orbital plane, Y = 0)
  const x = parentPos.x + Math.cos(angle) * orbitRadius;
  const z = parentPos.z + Math.sin(angle) * orbitRadius;
  const y = 0;

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

  // Only recurse if we haven't hit maxDepth
  if (node.children && node.children.length > 0 && depth < maxDepth) {
    // Classify children
    const { significantItems, asteroids } = classifyChildren(node.children);

    // Create asteroid belt for this node's small files
    if (asteroids.length > 0) {
      asteroidBelts.push({
        id: `belt-${currentPath}`,
        parentPath: currentPath,
        parentPos: { x, y, z },
        orbitRadius: 10, // Outer orbit relative to parent
        files: asteroids,
        count: asteroids.length,
        totalSize: asteroids.reduce((sum, a) => sum + a.size, 0),
      });
    }

    // Sort significant items for placement
    const sortedByModified = sortByLastModified(significantItems);
    const sortedByCreation = sortByCreatedAt(significantItems);

    sortedByModified.forEach((child, angleIndex) => {
      const radiusIndex = sortedByCreation.findIndex(c => c.name === child.name);
      const result = flattenWithPositions(
        child,
        depth + 1,
        angleIndex,
        radiusIndex,
        sortedByModified.length,
        { x, y, z },
        currentPath,
        maxDepth
      );
      items.push(...result.items);
      asteroidBelts.push(...result.asteroidBelts);
    });
  }

  return { items, asteroidBelts };
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

  // Navigation state for drill-down
  const [currentRoot, setCurrentRoot] = useState<FileNode>(sampleFileSystem);
  const [navigationPath, setNavigationPath] = useState<string[]>(['root']);

  // Camera control ref
  const universeRef = useRef<CameraControllerRef>(null);

  // Generate items and asteroid belts from current root
  const { items, asteroidBelts } = useMemo(() => flattenWithPositions(currentRoot), [currentRoot]);
  const totalSize = sampleFileSystem.size;
  const currentSize = currentRoot.size;

  // Drill-down handler
  const handleDrillDown = useCallback((item: PositionedItem) => {
    if (item.type === 'directory' && item.children && item.children.length > 0) {
      const node = findNodeByPath(sampleFileSystem, item.path);
      if (node) {
        setCurrentRoot(node);
        setNavigationPath(item.path.split('/').filter(Boolean));
        setSelectedItem(null);

        // Reset camera to overview after drill-down
        // Use setTimeout to allow React to re-render with new items first
        setTimeout(() => {
          universeRef.current?.resetView();
        }, 50);
      }
    }
  }, []);

  // File focus handler (when file is double-clicked)
  const handleFileFocus = useCallback((item: PositionedItem) => {
    setSelectedItem(item);
  }, []);

  // Navigate to specific level in breadcrumb
  const navigateToLevel = useCallback((index: number) => {
    if (index === 0) {
      setCurrentRoot(sampleFileSystem);
      setNavigationPath(['root']);
    } else {
      const targetPath = navigationPath.slice(0, index + 1).join('/');
      const node = findNodeByPath(sampleFileSystem, targetPath);
      if (node) {
        setCurrentRoot(node);
        setNavigationPath(navigationPath.slice(0, index + 1));
      }
    }
    setSelectedItem(null);

    // Reset camera to overview after navigation
    setTimeout(() => {
      universeRef.current?.resetView();
    }, 50);
  }, [navigationPath]);

  const selectedBreadcrumbs = selectedItem?.path.split('/').filter(Boolean) || [];

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
            ストレージを宇宙空間として可視化 - ドラッグで回転 - スクロールでズーム - ダブルクリックで中に入る
          </p>
        </div>

        {/* Navigation Breadcrumb */}
        <div style={{
          display: 'flex',
          alignItems: 'center',
          gap: '8px',
          marginBottom: '16px',
          padding: '12px 16px',
          background: 'rgba(255,255,255,0.03)',
          borderRadius: '12px',
          border: '1px solid rgba(255,255,255,0.1)',
          flexWrap: 'wrap'
        }}>
          <span style={{ fontSize: '12px', color: '#666', marginRight: '8px' }}>現在地:</span>
          {navigationPath.map((name, i) => (
            <span key={i} style={{ display: 'flex', alignItems: 'center' }}>
              {i > 0 && <span style={{ color: '#444', margin: '0 4px' }}>/</span>}
              <button
                onClick={() => navigateToLevel(i)}
                style={{
                  background: i === navigationPath.length - 1 ? 'rgba(168, 85, 247, 0.2)' : 'transparent',
                  border: 'none',
                  color: i === navigationPath.length - 1 ? '#a855f7' : '#888',
                  padding: '4px 8px',
                  borderRadius: '6px',
                  cursor: 'pointer',
                  fontSize: '13px',
                  fontWeight: i === navigationPath.length - 1 ? 'bold' : 'normal',
                  transition: 'all 0.2s'
                }}
                onMouseOver={(e) => e.currentTarget.style.background = 'rgba(168, 85, 247, 0.1)'}
                onMouseOut={(e) => e.currentTarget.style.background = i === navigationPath.length - 1 ? 'rgba(168, 85, 247, 0.2)' : 'transparent'}
              >
                {name === 'root' ? '🌟 ルート' : `📁 ${name}`}
              </button>
            </span>
          ))}
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
            <div style={{ fontSize: '24px', fontWeight: 'bold' }}>{formatSize(currentSize)}</div>
            <div style={{ fontSize: '12px', color: '#888' }}>現在のフォルダ</div>
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
          {/* 3D Canvas with overlay controls */}
          <div style={{ flex: '1 1 700px', position: 'relative' }}>
            <UniverseCanvas
              ref={universeRef}
              items={items}
              asteroidBelts={asteroidBelts}
              selectedItem={selectedItem}
              hoveredItem={hoveredItem}
              onSelect={setSelectedItem}
              onHover={setHoveredItem}
              onDrillDown={handleDrillDown}
              onFileFocus={handleFileFocus}
            />

            {/* Reset View Button */}
            <button
              onClick={() => {
                universeRef.current?.resetView();
              }}
              style={{
                position: 'absolute',
                bottom: '16px',
                right: '16px',
                padding: '8px 16px',
                background: 'rgba(168, 85, 247, 0.3)',
                border: '1px solid rgba(168, 85, 247, 0.5)',
                borderRadius: '8px',
                color: 'white',
                fontSize: '13px',
                cursor: 'pointer',
                backdropFilter: 'blur(4px)',
                display: 'flex',
                alignItems: 'center',
                gap: '6px',
                transition: 'all 0.2s',
              }}
              onMouseOver={(e) => {
                e.currentTarget.style.background = 'rgba(168, 85, 247, 0.5)';
              }}
              onMouseOut={(e) => {
                e.currentTarget.style.background = 'rgba(168, 85, 247, 0.3)';
              }}
            >
              Reset View
            </button>
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
                  {selectedBreadcrumbs.map((crumb, i) => (
                    <span key={i}>
                      {i > 0 && <span style={{ margin: '0 4px' }}>/</span>}
                      <span style={{ color: i === selectedBreadcrumbs.length - 1 ? '#a855f7' : '#888' }}>
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

                {/* Drill-down button for directories */}
                {selectedItem.type === 'directory' && selectedItem.children && selectedItem.children.length > 0 && (
                  <button
                    onClick={() => handleDrillDown(selectedItem)}
                    style={{
                      width: '100%',
                      padding: '12px',
                      background: 'linear-gradient(90deg, #a855f7, #3b82f6)',
                      border: 'none',
                      borderRadius: '8px',
                      color: 'white',
                      fontWeight: 'bold',
                      cursor: 'pointer',
                      marginBottom: '16px',
                      fontSize: '14px'
                    }}
                  >
                    🚀 このフォルダに入る
                  </button>
                )}

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
          <span>ダブルクリック: 中に入る</span>
        </div>
      </div>
    </div>
  );
}
