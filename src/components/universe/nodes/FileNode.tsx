'use client';

import { useRef, useState } from 'react';
import { useFrame } from '@react-three/fiber';
import { Html } from '@react-three/drei';
import * as THREE from 'three';
import { PositionedItem, typeColors, calculateRelativeRadius, calculateBrightness, formatSize } from '../types';

interface SizeRange {
  min: number;
  max: number;
}

interface FileNodeProps {
  item: PositionedItem;
  isSelected: boolean;
  isHovered: boolean;
  sizeRange: SizeRange;
  onSelect: (item: PositionedItem) => void;
  onHover: (item: PositionedItem | null) => void;
  onDoubleClick: (item: PositionedItem) => void;
  onFocus?: (position: [number, number, number]) => void;
}

export function FileNode({
  item,
  isSelected,
  isHovered,
  sizeRange,
  onSelect,
  onHover,
  onDoubleClick,
  onFocus,
}: FileNodeProps) {
  const meshRef = useRef<THREE.Mesh>(null);
  const [localHover, setLocalHover] = useState(false);

  const radius = calculateRelativeRadius(item.size, sizeRange.min, sizeRange.max);
  const color = typeColors[item.fileType || ''] || '#888888';
  const brightness = calculateBrightness(item.lastModified);

  // Subtle floating animation
  useFrame(({ clock }) => {
    if (meshRef.current) {
      const float = Math.sin(clock.elapsedTime * 2 + item.x) * 0.02;
      meshRef.current.position.y = item.y + float;
    }
  });

  const handlePointerOver = (e: any) => {
    e.stopPropagation();
    setLocalHover(true);
    onHover(item);
    document.body.style.cursor = 'pointer';
  };

  const handlePointerOut = (e: any) => {
    e.stopPropagation();
    setLocalHover(false);
    onHover(null);
    document.body.style.cursor = 'auto';
  };

  const handleClick = (e: any) => {
    e.stopPropagation();
    onSelect(item);
  };

  const handleDoubleClick = (e: any) => {
    e.stopPropagation();
    onDoubleClick(item);
  };

  const showLabel = isSelected || isHovered || localHover;
  const glowIntensity = isSelected ? 1.0 : isHovered || localHover ? 0.8 : brightness * 0.4 + 0.4;

  return (
    <group position={[item.x, item.y, item.z]}>
      {/* Main octahedron (file crystal) */}
      <mesh
        ref={meshRef}
        onPointerOver={handlePointerOver}
        onPointerOut={handlePointerOut}
        onClick={handleClick}
        onDoubleClick={handleDoubleClick}
      >
        <octahedronGeometry args={[radius, 0]} />
        <meshStandardMaterial
          color={color}
          emissive={color}
          emissiveIntensity={glowIntensity}
          roughness={0.3}
          metalness={0.5}
        />
      </mesh>

      {/* Selection ring */}
      {isSelected && (
        <mesh rotation={[Math.PI / 2, 0, 0]}>
          <ringGeometry args={[radius * 1.3, radius * 1.5, 16]} />
          <meshBasicMaterial color="#ffffff" transparent opacity={0.8} side={THREE.DoubleSide} />
        </mesh>
      )}

      {/* Label */}
      {showLabel && (
        <Html
          position={[0, radius + 0.3, 0]}
          center
          style={{
            pointerEvents: 'none',
            whiteSpace: 'nowrap',
          }}
        >
          <div
            style={{
              background: 'rgba(0, 0, 0, 0.8)',
              color: 'white',
              padding: '4px 8px',
              borderRadius: '4px',
              fontSize: '11px',
              fontFamily: 'system-ui, sans-serif',
            }}
          >
            <div style={{ fontWeight: 'bold', color }}>{item.name}</div>
            <div style={{ fontSize: '9px', color: '#888' }}>{formatSize(item.size)}</div>
          </div>
        </Html>
      )}

      {/* Focus button - shows when selected */}
      {isSelected && onFocus && (
        <Html
          position={[radius + 0.5, 0, 0]}
          center
        >
          <button
            onClick={(e) => {
              e.stopPropagation();
              onFocus([item.x, item.y, item.z]);
            }}
            style={{
              background: 'rgba(168, 85, 247, 0.8)',
              border: 'none',
              borderRadius: '50%',
              width: '28px',
              height: '28px',
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              fontSize: '12px',
              color: 'white',
              boxShadow: '0 2px 8px rgba(0,0,0,0.3)',
            }}
            title="Focus"
          >
            🎯
          </button>
        </Html>
      )}
    </group>
  );
}
