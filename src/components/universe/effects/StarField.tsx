'use client';

import { useRef, useMemo } from 'react';
import { useFrame } from '@react-three/fiber';
import * as THREE from 'three';

interface StarFieldProps {
  count?: number;
  radius?: number;
}

export function StarField({ count = 2000, radius = 100 }: StarFieldProps) {
  const pointsRef = useRef<THREE.Points>(null);

  // Generate random star positions
  const [positions, colors, sizes] = useMemo(() => {
    const positions = new Float32Array(count * 3);
    const colors = new Float32Array(count * 3);
    const sizes = new Float32Array(count);

    for (let i = 0; i < count; i++) {
      // Random position on a sphere
      const theta = Math.random() * Math.PI * 2;
      const phi = Math.acos(2 * Math.random() - 1);
      const r = radius * (0.5 + Math.random() * 0.5);

      positions[i * 3] = r * Math.sin(phi) * Math.cos(theta);
      positions[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta);
      positions[i * 3 + 2] = r * Math.cos(phi);

      // Slightly colored stars (white to blue/yellow tint)
      const colorVariation = Math.random();
      if (colorVariation < 0.7) {
        // White stars
        colors[i * 3] = 0.9 + Math.random() * 0.1;
        colors[i * 3 + 1] = 0.9 + Math.random() * 0.1;
        colors[i * 3 + 2] = 1;
      } else if (colorVariation < 0.85) {
        // Blue stars
        colors[i * 3] = 0.7;
        colors[i * 3 + 1] = 0.8;
        colors[i * 3 + 2] = 1;
      } else {
        // Yellow stars
        colors[i * 3] = 1;
        colors[i * 3 + 1] = 0.9;
        colors[i * 3 + 2] = 0.7;
      }

      // Random sizes
      sizes[i] = Math.random() * 1.5 + 0.5;
    }

    return [positions, colors, sizes];
  }, [count, radius]);

  // Twinkling animation
  useFrame(({ clock }) => {
    if (pointsRef.current) {
      const geometry = pointsRef.current.geometry;
      const sizesAttribute = geometry.getAttribute('size') as THREE.BufferAttribute;

      for (let i = 0; i < count; i++) {
        const baseSize = sizes[i];
        const twinkle = Math.sin(clock.elapsedTime * 2 + i * 0.1) * 0.3 + 0.7;
        sizesAttribute.setX(i, baseSize * twinkle);
      }
      sizesAttribute.needsUpdate = true;
    }
  });

  return (
    <points ref={pointsRef}>
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
        size={1}
        vertexColors
        transparent
        opacity={0.8}
        sizeAttenuation
        depthWrite={false}
      />
    </points>
  );
}
