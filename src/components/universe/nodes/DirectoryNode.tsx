'use client';

import { useRef, useState, useMemo } from 'react';
import { useFrame } from '@react-three/fiber';
import { Html } from '@react-three/drei';
import * as THREE from 'three';
import { PositionedItem, calculateNodeRadius, formatSize } from '../types';

interface DirectoryNodeProps {
  item: PositionedItem;
  isSelected: boolean;
  isHovered: boolean;
  onSelect: (item: PositionedItem) => void;
  onHover: (item: PositionedItem | null) => void;
  onDoubleClick: (item: PositionedItem) => void;
}

export function DirectoryNode({
  item,
  isSelected,
  isHovered,
  onSelect,
  onHover,
  onDoubleClick,
}: DirectoryNodeProps) {
  const meshRef = useRef<THREE.Mesh>(null);
  const ringRef = useRef<THREE.Mesh>(null);
  const [localHover, setLocalHover] = useState(false);

  const radius = calculateNodeRadius(item.size, 'directory');
  const color = '#ffffff'; // White for directory planets

  // Calculate ring tilt based on item name (deterministic random)
  const ringTilt = useMemo(() => {
    let hash = 0;
    for (let i = 0; i < item.name.length; i++) {
      hash = ((hash << 5) - hash) + item.name.charCodeAt(i);
      hash = hash & hash;
    }
    const tiltX = (Math.abs(hash % 100) / 100) * Math.PI * 0.4 + Math.PI * 0.2; // 0.2π to 0.6π
    const tiltZ = ((hash % 200) / 200) * Math.PI * 0.3; // 0 to 0.3π
    return { x: tiltX, z: tiltZ };
  }, [item.name]);

  // Calculate ring thickness based on child count
  const childCount = item.children?.length || 0;
  const ringThickness = radius * (0.08 + Math.min(childCount, 50) * 0.004); // Base + proportional to children

  // Pulse animation for directories
  useFrame(({ clock }) => {
    if (meshRef.current) {
      const pulse = 1 + Math.sin(clock.elapsedTime * 1.5) * 0.05;
      meshRef.current.scale.setScalar(pulse);
    }
    if (ringRef.current) {
      ringRef.current.rotation.z = clock.elapsedTime * 0.2;
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

  // Check if this is the star (center, depth=0)
  const isStar = item.depth === 0;

  // Only stars emit light, planets do not glow
  const glowIntensity = isStar
    ? (isSelected ? 2.0 : isHovered || localHover ? 1.8 : 1.5)
    : 0; // Planets: no emission

  return (
    <group position={[item.x, item.y, item.z]}>
      {/* Main sphere (planet) */}
      <mesh
        ref={meshRef}
        onPointerOver={handlePointerOver}
        onPointerOut={handlePointerOut}
        onClick={handleClick}
        onDoubleClick={handleDoubleClick}
      >
        <sphereGeometry args={[radius, 32, 32]} />
        <meshStandardMaterial
          color={color}
          emissive={color}
          emissiveIntensity={glowIntensity}
          roughness={0.2}
          metalness={0.1}
        />
      </mesh>

      {/* Ring (Saturn-like) - shows for planets only (not star) */}
      {!isStar && (
        <mesh ref={ringRef} rotation={[ringTilt.x, 0, ringTilt.z]}>
          <torusGeometry args={[radius * 1.6, ringThickness, 2, 64]} />
          <meshStandardMaterial
            color={color}
            transparent
            opacity={0.6}
            side={THREE.DoubleSide}
          />
        </mesh>
      )}

      {/* Selection ring */}
      {isSelected && (
        <mesh rotation={[Math.PI / 2, 0, 0]}>
          <ringGeometry args={[radius * 1.3, radius * 1.4, 32]} />
          <meshBasicMaterial color="#ffffff" transparent opacity={0.8} side={THREE.DoubleSide} />
        </mesh>
      )}

      {/* Label */}
      {showLabel && (
        <Html
          position={[0, radius + 0.5, 0]}
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
              fontSize: '12px',
              fontFamily: 'system-ui, sans-serif',
            }}
          >
            <div style={{ fontWeight: 'bold' }}>{item.name}</div>
            <div style={{ fontSize: '10px', color: '#888' }}>{formatSize(item.size)}</div>
          </div>
        </Html>
      )}
    </group>
  );
}
