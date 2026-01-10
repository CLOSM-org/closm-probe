'use client';

import { useState, useMemo, useCallback, useRef, useEffect } from 'react';
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
import {
  getAvailableRoots,
  readDirectoryStructure,
  getDirectorySize,
} from '@/app/actions/filesystem';

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
  maxDepth = 1
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

// Helper: shade color
function shadeColor(color: string, percent: number): string {
  const num = parseInt(color.replace('#', ''), 16);
  const amt = Math.round(2.55 * percent);
  const R = Math.min(255, Math.max(0, (num >> 16) + amt));
  const G = Math.min(255, Math.max(0, (num >> 8 & 0x00FF) + amt));
  const B = Math.min(255, Math.max(0, (num & 0x0000FF) + amt));
  return '#' + (0x1000000 + R * 0x10000 + G * 0x100 + B).toString(16).slice(1);
}

// Cache type for loaded directories
interface DirectoryCache {
  [path: string]: FileNode;
}

export default function PhysicalStorageUniverse() {
  const [selectedItem, setSelectedItem] = useState<PositionedItem | null>(null);
  const [hoveredItem, setHoveredItem] = useState<PositionedItem | null>(null);

  // File system state
  const [rootFileSystem, setRootFileSystem] = useState<FileNode | null>(null);
  const [availableRoots, setAvailableRoots] = useState<{ path: string; name: string }[]>([]);
  const [selectedRootPath, setSelectedRootPath] = useState<string>('');
  const [customPath, setCustomPath] = useState<string>('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [_showPathInput, setShowPathInput] = useState(false);

  // Cache for expanded directories
  const cacheRef = useRef<DirectoryCache>({});

  // Navigation state for drill-down
  const [currentRoot, setCurrentRoot] = useState<FileNode | null>(null);
  const [navigationPath, setNavigationPath] = useState<string[]>([]);

  // Camera control ref
  const universeRef = useRef<CameraControllerRef>(null);

  // Load available roots on mount
  useEffect(() => {
    const loadRoots = async () => {
      const result = await getAvailableRoots();
      if (result.success && result.roots) {
        setAvailableRoots(result.roots);
      }
    };
    loadRoots();
  }, []);

  // Size cache: stores calculated sizes by full path
  const sizeCacheRef = useRef<{ [path: string]: number }>({});

  // Load directory structure (fast, without recursive size calculation)
  const loadDirectory = useCallback(async (dirPath: string, depth: number = 2) => {
    // Check structure cache first
    if (cacheRef.current[dirPath]) {
      return cacheRef.current[dirPath];
    }

    setIsLoading(true);
    setError(null);

    try {
      // Use structure-only loading for fast navigation
      const result = await readDirectoryStructure(dirPath, depth);
      if (result.success && result.data) {
        cacheRef.current[dirPath] = result.data;
        return result.data;
      } else {
        setError(result.error || 'Failed to read directory');
        return null;
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
      return null;
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Calculate sizes for directories in background
  const calculateSizesInBackground = useCallback(async (
    rootPath: string,
    node: FileNode,
    onSizeUpdate: () => void
  ) => {
    // Build full path for this node
    const nodePath = rootPath;

    // Helper to recursively calculate sizes
    const processNode = async (currentNode: FileNode, currentPath: string) => {
      if (currentNode.type === 'file') {
        return; // Files already have size from stat
      }

      // Check cache first
      if (sizeCacheRef.current[currentPath] !== undefined) {
        currentNode.size = sizeCacheRef.current[currentPath];
        onSizeUpdate();
        return;
      }

      // Calculate size for this directory
      try {
        const result = await getDirectorySize(currentPath);
        if (result.success && result.size !== undefined) {
          currentNode.size = result.size;
          sizeCacheRef.current[currentPath] = result.size;
          onSizeUpdate();
        }
      } catch {
        // Keep size as unknown on error
      }

      // Process children (but don't wait - let them run in parallel)
      if (currentNode.children) {
        for (const child of currentNode.children) {
          if (child.type === 'directory') {
            const childPath = `${currentPath}/${child.name}`;
            // Don't await - let it run in background
            processNode(child, childPath);
          }
        }
      }
    };

    await processNode(node, nodePath);
  }, []);

  // Handle root selection
  const handleSelectRoot = useCallback(async (rootPath: string) => {
    setSelectedRootPath(rootPath);
    setShowPathInput(false);

    const data = await loadDirectory(rootPath);
    if (data) {
      setRootFileSystem(data);
      setCurrentRoot(data);
      setNavigationPath([data.name]);
      setSelectedItem(null);

      setTimeout(() => {
        universeRef.current?.resetView();
      }, 50);

      // Start background size calculation
      calculateSizesInBackground(rootPath, data, () => {
        // Force re-render when sizes are updated
        setRootFileSystem(prev => prev ? { ...prev } : null);
      });
    }
  }, [loadDirectory, calculateSizesInBackground]);

  // Handle custom path input
  const handleCustomPathSubmit = useCallback(async () => {
    if (customPath.trim()) {
      await handleSelectRoot(customPath.trim());
    }
  }, [customPath, handleSelectRoot]);

  // Generate items and asteroid belts from current root
  const { items, asteroidBelts } = useMemo(() => {
    if (!currentRoot) {
      return { items: [], asteroidBelts: [] };
    }
    return flattenWithPositions(currentRoot);
  }, [currentRoot]);

  const totalSize = rootFileSystem?.size || 0;
  const currentSize = currentRoot?.size || 0;

  // Calculate size range for planets (depth 1) for relative sizing
  const sizeRange = useMemo(() => {
    const planetSizes = items
      .filter(item => item.depth === 1)
      .map(item => item.size);
    if (planetSizes.length === 0) {
      return { min: 0, max: 0 };
    }
    return {
      min: Math.min(...planetSizes),
      max: Math.max(...planetSizes),
    };
  }, [items]);

  // Drill-down handler with lazy loading (unlimited depth)
  const handleDrillDown = useCallback(async (item: PositionedItem) => {
    if (!rootFileSystem) return;

    if (item.type === 'directory') {
      // Find the node in current tree
      const node = findNodeByPath(rootFileSystem, item.path);
      if (node) {
        // Build full path from root
        const pathParts = item.path.split('/').slice(1);
        const fullPath = pathParts.length > 0
          ? selectedRootPath + '/' + pathParts.join('/')
          : selectedRootPath;

        // If node has no children loaded yet (or empty children), load them
        if (!node.children || node.children.length === 0) {
          const expanded = await loadDirectory(fullPath, 2);
          if (expanded) {
            // Merge expanded data into the tree
            node.children = expanded.children;

            // Start background size calculation for new children
            calculateSizesInBackground(fullPath, node, () => {
              setRootFileSystem(prev => prev ? { ...prev } : null);
            });
          }
        }

        // Navigate even if children couldn't be loaded
        if (node.children && node.children.length > 0) {
          setCurrentRoot(node);
          setNavigationPath(item.path.split('/').filter(Boolean));
          setSelectedItem(null);

          // Reset camera to overview after drill-down
          setTimeout(() => {
            universeRef.current?.resetView();
          }, 50);
        }
      }
    }
  }, [rootFileSystem, selectedRootPath, loadDirectory, calculateSizesInBackground]);

  // File focus handler (when file is double-clicked)
  const handleFileFocus = useCallback((item: PositionedItem) => {
    setSelectedItem(item);
  }, []);

  // Navigate to specific level in breadcrumb
  const navigateToLevel = useCallback((index: number) => {
    if (!rootFileSystem) return;

    if (index === 0) {
      setCurrentRoot(rootFileSystem);
      setNavigationPath([rootFileSystem.name]);
    } else {
      const targetPath = navigationPath.slice(0, index + 1).join('/');
      const node = findNodeByPath(rootFileSystem, targetPath);
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
  }, [navigationPath, rootFileSystem]);

  const selectedBreadcrumbs = selectedItem?.path.split('/').filter(Boolean) || [];

  // Directory selection screen when no root is selected
  if (!rootFileSystem) {
    return (
      <div style={{
        minHeight: '100vh',
        background: 'linear-gradient(180deg, #0a0a1a 0%, #1a1a2e 100%)',
        color: 'white',
        padding: '20px',
        fontFamily: 'system-ui, sans-serif',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center'
      }}>
        <div style={{ maxWidth: '600px', width: '100%', textAlign: 'center' }}>
          <h1 style={{
            fontSize: '48px',
            fontWeight: 'bold',
            background: 'linear-gradient(90deg, #a855f7, #3b82f6, #06b6d4)',
            WebkitBackgroundClip: 'text',
            WebkitTextFillColor: 'transparent',
            marginBottom: '16px'
          }}>
            Storage Universe
          </h1>
          <p style={{ color: '#888', fontSize: '16px', marginBottom: '40px' }}>
            ローカルストレージを宇宙空間として可視化
          </p>

          {error && (
            <div style={{
              background: 'rgba(239, 68, 68, 0.1)',
              border: '1px solid rgba(239, 68, 68, 0.3)',
              borderRadius: '12px',
              padding: '12px 16px',
              marginBottom: '20px',
              color: '#ef4444',
              fontSize: '14px'
            }}>
              {error}
            </div>
          )}

          {isLoading ? (
            <div style={{
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              gap: '16px'
            }}>
              <div style={{
                width: '48px',
                height: '48px',
                border: '3px solid rgba(168, 85, 247, 0.3)',
                borderTopColor: '#a855f7',
                borderRadius: '50%',
                animation: 'spin 1s linear infinite'
              }} />
              <style>{`
                @keyframes spin {
                  to { transform: rotate(360deg); }
                }
              `}</style>
              <p style={{ color: '#888' }}>ディレクトリを読み込み中...</p>
            </div>
          ) : (
            <>
              <div style={{
                background: 'rgba(255,255,255,0.03)',
                borderRadius: '16px',
                padding: '24px',
                border: '1px solid rgba(255,255,255,0.1)',
                marginBottom: '20px'
              }}>
                <h2 style={{ fontSize: '18px', marginBottom: '16px', color: '#fff' }}>
                  ディレクトリを選択
                </h2>
                <div style={{
                  display: 'grid',
                  gridTemplateColumns: 'repeat(auto-fit, minmax(140px, 1fr))',
                  gap: '12px'
                }}>
                  {availableRoots.map((root) => (
                    <button
                      key={root.path}
                      onClick={() => handleSelectRoot(root.path)}
                      style={{
                        padding: '16px',
                        background: 'rgba(168, 85, 247, 0.1)',
                        border: '1px solid rgba(168, 85, 247, 0.3)',
                        borderRadius: '12px',
                        color: 'white',
                        cursor: 'pointer',
                        transition: 'all 0.2s',
                        textAlign: 'center'
                      }}
                      onMouseOver={(e) => {
                        e.currentTarget.style.background = 'rgba(168, 85, 247, 0.2)';
                        e.currentTarget.style.borderColor = 'rgba(168, 85, 247, 0.5)';
                      }}
                      onMouseOut={(e) => {
                        e.currentTarget.style.background = 'rgba(168, 85, 247, 0.1)';
                        e.currentTarget.style.borderColor = 'rgba(168, 85, 247, 0.3)';
                      }}
                    >
                      <div style={{ fontSize: '24px', marginBottom: '8px' }}>
                        {root.name === 'Home' ? '🏠' :
                         root.name === 'Documents' ? '📄' :
                         root.name === 'Downloads' ? '⬇️' :
                         root.name === 'Desktop' ? '🖥️' :
                         root.name === 'Work' ? '💼' :
                         root.name === 'Projects' ? '🚀' :
                         root.name === 'Temp' ? '🗑️' : '📁'}
                      </div>
                      <div style={{ fontSize: '14px', fontWeight: 'bold' }}>{root.name}</div>
                    </button>
                  ))}
                </div>
              </div>

              <div style={{
                background: 'rgba(255,255,255,0.03)',
                borderRadius: '16px',
                padding: '24px',
                border: '1px solid rgba(255,255,255,0.1)'
              }}>
                <h2 style={{ fontSize: '18px', marginBottom: '16px', color: '#fff' }}>
                  カスタムパス
                </h2>
                <div style={{ display: 'flex', gap: '12px' }}>
                  <input
                    type="text"
                    value={customPath}
                    onChange={(e) => setCustomPath(e.target.value)}
                    placeholder="/path/to/directory"
                    onKeyDown={(e) => {
                      if (e.key === 'Enter') {
                        handleCustomPathSubmit();
                      }
                    }}
                    style={{
                      flex: 1,
                      padding: '12px 16px',
                      background: 'rgba(255,255,255,0.05)',
                      border: '1px solid rgba(255,255,255,0.1)',
                      borderRadius: '8px',
                      color: 'white',
                      fontSize: '14px',
                      outline: 'none'
                    }}
                  />
                  <button
                    onClick={handleCustomPathSubmit}
                    disabled={!customPath.trim()}
                    style={{
                      padding: '12px 24px',
                      background: customPath.trim() ? 'linear-gradient(90deg, #a855f7, #3b82f6)' : 'rgba(255,255,255,0.1)',
                      border: 'none',
                      borderRadius: '8px',
                      color: 'white',
                      fontSize: '14px',
                      fontWeight: 'bold',
                      cursor: customPath.trim() ? 'pointer' : 'not-allowed',
                      opacity: customPath.trim() ? 1 : 0.5
                    }}
                  >
                    開く
                  </button>
                </div>
              </div>
            </>
          )}
        </div>
      </div>
    );
  }

  return (
    <div style={{
      minHeight: '100vh',
      background: 'linear-gradient(180deg, #0a0a1a 0%, #1a1a2e 100%)',
      color: 'white',
      padding: '20px',
      fontFamily: 'system-ui, sans-serif'
    }}>
      <div style={{ maxWidth: '1200px', margin: '0 auto' }}>
        {/* Header with directory selector */}
        <div style={{ marginBottom: '20px', display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start', flexWrap: 'wrap', gap: '16px' }}>
          <div>
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
              ドラッグで回転 - スクロールでズーム - ダブルクリックで中に入る
            </p>
          </div>
          <button
            onClick={() => {
              setRootFileSystem(null);
              setCurrentRoot(null);
              setNavigationPath([]);
              setSelectedItem(null);
              setError(null);
            }}
            style={{
              padding: '8px 16px',
              background: 'rgba(255,255,255,0.05)',
              border: '1px solid rgba(255,255,255,0.1)',
              borderRadius: '8px',
              color: '#888',
              fontSize: '13px',
              cursor: 'pointer',
              transition: 'all 0.2s'
            }}
            onMouseOver={(e) => {
              e.currentTarget.style.background = 'rgba(255,255,255,0.1)';
              e.currentTarget.style.color = '#fff';
            }}
            onMouseOut={(e) => {
              e.currentTarget.style.background = 'rgba(255,255,255,0.05)';
              e.currentTarget.style.color = '#888';
            }}
          >
            別のフォルダを選択
          </button>
        </div>

        {/* Loading indicator */}
        {isLoading && (
          <div style={{
            position: 'fixed',
            top: '20px',
            right: '20px',
            background: 'rgba(168, 85, 247, 0.2)',
            border: '1px solid rgba(168, 85, 247, 0.3)',
            borderRadius: '8px',
            padding: '8px 16px',
            fontSize: '13px',
            color: '#a855f7',
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            zIndex: 1000
          }}>
            <div style={{
              width: '14px',
              height: '14px',
              border: '2px solid rgba(168, 85, 247, 0.3)',
              borderTopColor: '#a855f7',
              borderRadius: '50%',
              animation: 'spin 1s linear infinite'
            }} />
            読み込み中...
          </div>
        )}

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
                {i === 0 ? '🌟 ' + name : '📁 ' + name}
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
              sizeRange={sizeRange}
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
