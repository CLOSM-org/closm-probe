'use client';

import { useRef, useState, useMemo } from 'react';
import { useFrame } from '@react-three/fiber';
import { Html } from '@react-three/drei';
import * as THREE from 'three';
import { PositionedItem, calculateRelativeRadius, formatSize, STAR_RADIUS } from '../types';

interface SizeRange {
  min: number;
  max: number;
}

interface DirectoryNodeProps {
  item: PositionedItem;
  isSelected: boolean;
  isHovered: boolean;
  sizeRange: SizeRange;
  onSelect: (item: PositionedItem) => void;
  onHover: (item: PositionedItem | null) => void;
  onDoubleClick: (item: PositionedItem) => void;
  onFocus?: (position: [number, number, number]) => void;
}

export function DirectoryNode({
  item,
  isSelected,
  isHovered,
  sizeRange,
  onSelect,
  onHover,
  onDoubleClick,
  onFocus,
}: DirectoryNodeProps) {
  const meshRef = useRef<THREE.Mesh>(null);
  const ringRef = useRef<THREE.Mesh>(null);
  const [localHover, setLocalHover] = useState(false);
  const [satelliteScale, setSatelliteScale] = useState(0);

  // Check if this is the star (center, depth=0) - used for radius and glow
  const isStar = item.depth === 0;

  // Star has fixed radius (largest), planets use relative sizing
  const radius = isStar
    ? STAR_RADIUS
    : calculateRelativeRadius(item.size, sizeRange.min, sizeRange.max);
  const color = '#ffffff'; // White for directory planets/star

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

  // Pulse animation for directories and satellite expansion
  useFrame(({ clock }, delta) => {
    if (meshRef.current) {
      const pulse = 1 + Math.sin(clock.elapsedTime * 1.5) * 0.05;
      meshRef.current.scale.setScalar(pulse);
    }
    if (ringRef.current) {
      ringRef.current.rotation.z = clock.elapsedTime * 0.2;
    }
    // Animate satellite scale on hover
    if (localHover && satelliteScale < 1) {
      setSatelliteScale(prev => Math.min(1, prev + delta * 4));
    } else if (!localHover && satelliteScale > 0) {
      setSatelliteScale(prev => Math.max(0, prev - delta * 4));
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

      {/* Focus button - shows when selected */}
      {isSelected && onFocus && (
        <Html
          position={[radius + 0.8, 0, 0]}
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
              width: '32px',
              height: '32px',
              cursor: 'pointer',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              fontSize: '14px',
              color: 'white',
              boxShadow: '0 2px 8px rgba(0,0,0,0.3)',
            }}
            title="Focus"
          >
            🎯
          </button>
        </Html>
      )}

      {/* Satellites - show on hover with animation (not for star) */}
      {!isStar && satelliteScale > 0 && item.children && item.children.length > 0 && (
        <group rotation={[ringTilt.x, 0, ringTilt.z]}>
          {item.children.slice(0, 12).map((child, index) => {
            const satelliteAngle = (index / Math.min(item.children!.length, 12)) * Math.PI * 2;
            const satelliteRadius = radius * 2.5;
            const satelliteSize = (child.type === 'directory' ? 0.3 : 0.25) * satelliteScale;
            const sx = Math.cos(satelliteAngle) * satelliteRadius;
            const sz = Math.sin(satelliteAngle) * satelliteRadius;

            return (
              <mesh key={child.name} position={[sx, 0, sz]} scale={satelliteScale}>
                {child.type === 'directory' ? (
                  <sphereGeometry args={[satelliteSize / satelliteScale, 8, 8]} />
                ) : (
                  <octahedronGeometry args={[satelliteSize / satelliteScale, 0]} />
                )}
                <meshStandardMaterial
                  color={child.type === 'directory' ? '#ffffff' : '#61dafb'}
                  emissive={child.type === 'directory' ? '#ffffff' : '#61dafb'}
                  emissiveIntensity={0.3}
                  transparent
                  opacity={satelliteScale}
                />
              </mesh>
            );
          })}
        </group>
      )}
    </group>
  );
}
