'use client';

import { useMemo } from 'react';
import { Line } from '@react-three/drei';
import * as THREE from 'three';
import { PositionedItem } from '../types';

interface OrbitLinesProps {
  items: PositionedItem[];
}

export function OrbitLines({ items }: OrbitLinesProps) {
  // Generate orbit circles for directories
  const orbits = useMemo(() => {
    return items
      .filter(item => item.type === 'directory' && item.depth > 0 && item.parentPos)
      .map(item => {
        const { parentPos, orbitRadius } = item;
        if (!parentPos) return null;

        // Generate circle points
        const segments = 64;
        const points: THREE.Vector3[] = [];

        for (let i = 0; i <= segments; i++) {
          const angle = (i / segments) * Math.PI * 2;
          points.push(
            new THREE.Vector3(
              parentPos.x + Math.cos(angle) * orbitRadius,
              parentPos.y,
              parentPos.z + Math.sin(angle) * orbitRadius
            )
          );
        }

        return {
          key: item.path,
          points,
        };
      })
      .filter(Boolean);
  }, [items]);

  return (
    <>
      {orbits.map(orbit => orbit && (
        <Line
          key={orbit.key}
          points={orbit.points}
          color="#4444aa"
          opacity={0.2}
          transparent
          lineWidth={1}
          dashed
          dashSize={0.3}
          gapSize={0.2}
        />
      ))}
    </>
  );
}
