'use client';

import { useRef, useMemo } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';
import { Html } from '@react-three/drei';
import { AsteroidBelt as AsteroidBeltType, formatSize } from '../types';

interface AsteroidBeltProps {
  belt: AsteroidBeltType;
  isHovered: boolean;
  onHover: (belt: AsteroidBeltType | null) => void;
  onClick: (belt: AsteroidBeltType) => void;
}

export function AsteroidBeltRing({ belt, isHovered, onHover, onClick }: AsteroidBeltProps) {
  const ringRef = useRef<THREE.Points>(null);
  const particleCount = Math.min(belt.count * 3, 100); // Max 100 particles

  // Generate asteroid particles around the orbit
  const [positions, colors, sizes] = useMemo(() => {
    const positions = new Float32Array(particleCount * 3);
    const colors = new Float32Array(particleCount * 3);
    const sizes = new Float32Array(particleCount);

    for (let i = 0; i < particleCount; i++) {
      // Distribute around the orbit with some variation
      const angle = (i / particleCount) * Math.PI * 2;
      const radiusVariation = belt.orbitRadius * (0.9 + Math.random() * 0.2);
      const heightVariation = (Math.random() - 0.5) * 0.3;

      positions[i * 3] = belt.parentPos.x + Math.cos(angle) * radiusVariation;
      positions[i * 3 + 1] = belt.parentPos.y + heightVariation;
      positions[i * 3 + 2] = belt.parentPos.z + Math.sin(angle) * radiusVariation;

      // Gray/brown asteroid colors
      const colorVariation = 0.3 + Math.random() * 0.3;
      colors[i * 3] = colorVariation;
      colors[i * 3 + 1] = colorVariation * 0.9;
      colors[i * 3 + 2] = colorVariation * 0.8;

      // Random sizes
      sizes[i] = 2 + Math.random() * 3;
    }

    return [positions, colors, sizes];
  }, [belt, particleCount]);

  // Slow rotation animation
  useFrame(({ clock }) => {
    if (ringRef.current) {
      ringRef.current.rotation.y = clock.elapsedTime * 0.02;
    }
  });

  return (
    <group>
      <points
        ref={ringRef}
        onPointerOver={(e) => {
          e.stopPropagation();
          onHover(belt);
        }}
        onPointerOut={() => onHover(null)}
        onClick={(e) => {
          e.stopPropagation();
          onClick(belt);
        }}
      >
        <bufferGeometry>
          <bufferAttribute
            attach="attributes-position"
            args={[positions, 3]}
          />
          <bufferAttribute
            attach="attributes-color"
            args={[colors, 3]}
          />
          <bufferAttribute
            attach="attributes-size"
            args={[sizes, 1]}
          />
        </bufferGeometry>
        <pointsMaterial
          size={isHovered ? 4 : 2}
          vertexColors
          transparent
          opacity={isHovered ? 0.9 : 0.6}
          sizeAttenuation
          depthWrite={false}
        />
      </points>

      {/* Count badge */}
      <Html
        position={[
          belt.parentPos.x + belt.orbitRadius,
          belt.parentPos.y + 0.5,
          belt.parentPos.z
        ]}
        center
        style={{ pointerEvents: 'none' }}
      >
        <div
          style={{
            background: isHovered ? 'rgba(168, 85, 247, 0.9)' : 'rgba(100, 100, 100, 0.8)',
            color: 'white',
            padding: '2px 6px',
            borderRadius: '10px',
            fontSize: '10px',
            fontWeight: 'bold',
            whiteSpace: 'nowrap',
            transition: 'all 0.2s',
          }}
        >
          +{belt.count} files ({formatSize(belt.totalSize)})
        </div>
      </Html>

      {/* Hover ring highlight */}
      {isHovered && (
        <mesh
          position={[belt.parentPos.x, belt.parentPos.y, belt.parentPos.z]}
          rotation={[-Math.PI / 2, 0, 0]}
        >
          <ringGeometry args={[belt.orbitRadius * 0.85, belt.orbitRadius * 1.15, 64]} />
          <meshBasicMaterial
            color="#a855f7"
            transparent
            opacity={0.2}
            side={THREE.DoubleSide}
          />
        </mesh>
      )}
    </group>
  );
}
