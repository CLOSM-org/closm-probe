'use client';

import { useRef, Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import { CameraController, CameraControllerRef } from './controls/CameraController';
import { StarField } from './effects/StarField';
import { OrbitLines } from './effects/OrbitLines';
import { DirectoryNode } from './nodes/DirectoryNode';
import { FileNode } from './nodes/FileNode';
import { Effects } from './postprocessing/Effects';
import { PositionedItem } from './types';

interface UniverseCanvasProps {
  items: PositionedItem[];
  selectedItem: PositionedItem | null;
  hoveredItem: PositionedItem | null;
  onSelect: (item: PositionedItem | null) => void;
  onHover: (item: PositionedItem | null) => void;
  onDrillDown?: (item: PositionedItem) => void;
}

function Scene({
  items,
  selectedItem,
  hoveredItem,
  onSelect,
  onHover,
  onDrillDown,
  cameraRef,
}: UniverseCanvasProps & { cameraRef: React.RefObject<CameraControllerRef> }) {
  const handleDoubleClick = (item: PositionedItem) => {
    // For directories with children, drill down
    if (item.type === 'directory' && item.children && item.children.length > 0 && onDrillDown) {
      onDrillDown(item);
    } else if (cameraRef.current) {
      // For files or empty directories, just focus camera
      cameraRef.current.focusOn([item.x, item.y, item.z]);
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
      <StarField count={3000} radius={80} />

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

      {/* Post-processing effects */}
      <Effects />
    </>
  );
}

export function UniverseCanvas({
  items,
  selectedItem,
  hoveredItem,
  onSelect,
  onHover,
  onDrillDown,
}: UniverseCanvasProps) {
  const cameraRef = useRef<CameraControllerRef>(null);

  // Click on empty space deselects
  const handleCanvasClick = () => {
    // This is handled by node click stopPropagation
  };

  return (
    <div style={{ width: '100%', height: '500px', borderRadius: '16px', overflow: 'hidden' }}>
      <Canvas
        camera={{ position: [0, 8, 15], fov: 60 }}
        style={{ background: 'linear-gradient(180deg, #0a0a1a 0%, #1a1a2e 100%)' }}
        onClick={handleCanvasClick}
      >
        <Suspense fallback={null}>
          <Scene
            items={items}
            selectedItem={selectedItem}
            hoveredItem={hoveredItem}
            onSelect={onSelect}
            onHover={onHover}
            onDrillDown={onDrillDown}
            cameraRef={cameraRef}
          />
        </Suspense>
      </Canvas>
    </div>
  );
}
