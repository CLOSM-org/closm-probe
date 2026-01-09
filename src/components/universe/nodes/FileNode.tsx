'use client';

import { useRef, useState } from 'react';
import { useFrame } from '@react-three/fiber';
import { Html } from '@react-three/drei';
import * as THREE from 'three';
import { PositionedItem, typeColors, calculateNodeRadius, calculateBrightness, formatSize } from '../types';

interface FileNodeProps {
  item: PositionedItem;
  isSelected: boolean;
  isHovered: boolean;
  onSelect: (item: PositionedItem) => void;
  onHover: (item: PositionedItem | null) => void;
  onDoubleClick: (item: PositionedItem) => void;
}

export function FileNode({
  item,
  isSelected,
  isHovered,
  onSelect,
  onHover,
  onDoubleClick,
}: FileNodeProps) {
  const meshRef = useRef<THREE.Mesh>(null);
  const [localHover, setLocalHover] = useState(false);

  const radius = calculateNodeRadius(item.size, 'file');
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
      {/* Main sphere (satellite) */}
      <mesh
        ref={meshRef}
        onPointerOver={handlePointerOver}
        onPointerOut={handlePointerOut}
        onClick={handleClick}
        onDoubleClick={handleDoubleClick}
      >
        <sphereGeometry args={[radius, 16, 16]} />
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
    </group>
  );
}
