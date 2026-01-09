'use client';

import { useRef, useState, useCallback, Suspense, forwardRef, useImperativeHandle } from 'react';
import { Canvas } from '@react-three/fiber';
import { CameraController, CameraControllerRef } from './controls/CameraController';
import { StarField } from './effects/StarField';
import { OrbitLines } from './effects/OrbitLines';
import { DirectoryNode } from './nodes/DirectoryNode';
import { FileNode } from './nodes/FileNode';
import { AsteroidBeltRing } from './nodes/AsteroidBelt';
import { Effects } from './postprocessing/Effects';
import { PositionedItem, AsteroidBelt } from './types';

interface UniverseCanvasProps {
  items: PositionedItem[];
  asteroidBelts: AsteroidBelt[];
  selectedItem: PositionedItem | null;
  hoveredItem: PositionedItem | null;
  onSelect: (item: PositionedItem | null) => void;
  onHover: (item: PositionedItem | null) => void;
  onDrillDown?: (item: PositionedItem) => void;
  onFileFocus?: (item: PositionedItem) => void;
}

function Scene({
  items,
  asteroidBelts,
  selectedItem,
  hoveredItem,
  onSelect,
  onHover,
  onDrillDown,
  onFileFocus,
  cameraRef,
  onBeltHover,
  onBeltClick,
  hoveredBeltId,
}: UniverseCanvasProps & {
  cameraRef: React.RefObject<CameraControllerRef | null>;
  onBeltHover: (belt: AsteroidBelt | null) => void;
  onBeltClick: (belt: AsteroidBelt) => void;
  hoveredBeltId: string | null;
}) {
  const handleDoubleClick = (item: PositionedItem) => {
    // For directories with children, drill down
    if (item.type === 'directory' && item.children && item.children.length > 0 && onDrillDown) {
      onDrillDown(item);
    } else {
      // For files or empty directories, focus camera
      if (cameraRef.current) {
        cameraRef.current.focusOn([item.x, item.y, item.z]);
      }
      // Notify parent about file focus
      if (onFileFocus) {
        onFileFocus(item);
      }
    }
  };

  return (
    <>
      {/* Lighting */}
      <ambientLight intensity={0.3} />
      <pointLight position={[10, 10, 10]} intensity={1} />
      <pointLight position={[-10, -10, -10]} intensity={0.5} color="#6666ff" />

      {/* Camera controls */}
      <CameraController ref={cameraRef} />

      {/* Background stars */}
      <StarField count={1500} radius={200} />

      {/* Orbit lines */}
      <OrbitLines items={items} />

      {/* Nodes */}
      {items.map(item =>
        item.type === 'directory' ? (
          <DirectoryNode
            key={item.path}
            item={item}
            isSelected={selectedItem?.path === item.path}
            isHovered={hoveredItem?.path === item.path}
            onSelect={onSelect}
            onHover={onHover}
            onDoubleClick={handleDoubleClick}
          />
        ) : (
          <FileNode
            key={item.path}
            item={item}
            isSelected={selectedItem?.path === item.path}
            isHovered={hoveredItem?.path === item.path}
            onSelect={onSelect}
            onHover={onHover}
            onDoubleClick={handleDoubleClick}
          />
        )
      )}

      {/* Asteroid belts */}
      {asteroidBelts.map(belt => (
        <AsteroidBeltRing
          key={belt.id}
          belt={belt}
          isHovered={hoveredBeltId === belt.id}
          onHover={onBeltHover}
          onClick={onBeltClick}
        />
      ))}

      {/* Post-processing effects */}
      <Effects />
    </>
  );
}

export const UniverseCanvas = forwardRef<CameraControllerRef, UniverseCanvasProps>(
  function UniverseCanvas({
    items,
    asteroidBelts,
    selectedItem,
    hoveredItem,
    onSelect,
    onHover,
    onDrillDown,
    onFileFocus,
  }, ref) {
    const cameraRef = useRef<CameraControllerRef>(null);
    const [hoveredBeltId, setHoveredBeltId] = useState<string | null>(null);

    // Expose camera methods to parent
    useImperativeHandle(ref, () => ({
      focusOn: (pos: [number, number, number]) => cameraRef.current?.focusOn(pos),
      resetView: () => cameraRef.current?.resetView(),
    }));

    // Click on empty space deselects
    const handleCanvasClick = () => {
      // This is handled by node click stopPropagation
    };

    // Asteroid belt handlers
    const handleBeltHover = useCallback((belt: AsteroidBelt | null) => {
      setHoveredBeltId(belt?.id || null);
    }, []);

    const handleBeltClick = useCallback((belt: AsteroidBelt) => {
      // For now, just log the belt info - could expand to show detail panel
      console.log('Asteroid belt clicked:', belt.count, 'files', belt.totalSize, 'bytes');
    }, []);

    return (
      <div style={{ width: '100%', height: '500px', borderRadius: '16px', overflow: 'hidden' }}>
        <Canvas
          camera={{ position: [0, 12, 22], fov: 55 }}
          style={{ background: '#000000' }}
          onClick={handleCanvasClick}
        >
          <Suspense fallback={null}>
            <Scene
              items={items}
              asteroidBelts={asteroidBelts}
              selectedItem={selectedItem}
              hoveredItem={hoveredItem}
              onSelect={onSelect}
              onHover={onHover}
              onDrillDown={onDrillDown}
              onFileFocus={onFileFocus}
              cameraRef={cameraRef}
              onBeltHover={handleBeltHover}
              onBeltClick={handleBeltClick}
              hoveredBeltId={hoveredBeltId}
            />
          </Suspense>
        </Canvas>
      </div>
    );
  }
);

UniverseCanvas.displayName = 'UniverseCanvas';
