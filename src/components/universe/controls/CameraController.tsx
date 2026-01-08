'use client';

import { useRef, useEffect, forwardRef, useImperativeHandle } from 'react';
import { useThree, useFrame } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import * as THREE from 'three';

export interface CameraControllerRef {
  focusOn: (position: [number, number, number]) => void;
}

interface CameraControllerProps {
  enableDamping?: boolean;
  dampingFactor?: number;
  minDistance?: number;
  maxDistance?: number;
}

export const CameraController = forwardRef<CameraControllerRef, CameraControllerProps>(
  ({ enableDamping = true, dampingFactor = 0.05, minDistance = 3, maxDistance = 50 }, ref) => {
    const controlsRef = useRef<any>(null);
    const { camera } = useThree();

    // Target position for smooth camera animation
    const targetPosition = useRef<THREE.Vector3 | null>(null);
    const targetLookAt = useRef<THREE.Vector3 | null>(null);
    const isAnimating = useRef(false);

    useImperativeHandle(ref, () => ({
      focusOn: (position: [number, number, number]) => {
        const [x, y, z] = position;
        // Set target to look at the position
        targetLookAt.current = new THREE.Vector3(x, y, z);
        // Set camera target position (offset from the node)
        const offset = new THREE.Vector3(x, y + 2, z + 5);
        targetPosition.current = offset;
        isAnimating.current = true;
      },
    }));

    useFrame(() => {
      if (isAnimating.current && targetPosition.current && targetLookAt.current) {
        // Smoothly lerp camera position
        camera.position.lerp(targetPosition.current, 0.05);

        // Update orbit controls target
        if (controlsRef.current) {
          controlsRef.current.target.lerp(targetLookAt.current, 0.05);
        }

        // Check if close enough to stop animating
        if (camera.position.distanceTo(targetPosition.current) < 0.1) {
          isAnimating.current = false;
          targetPosition.current = null;
          targetLookAt.current = null;
        }
      }
    });

    return (
      <OrbitControls
        ref={controlsRef}
        enableDamping={enableDamping}
        dampingFactor={dampingFactor}
        minDistance={minDistance}
        maxDistance={maxDistance}
        enablePan={true}
        enableZoom={true}
        enableRotate={true}
        maxPolarAngle={Math.PI * 0.9}
        minPolarAngle={Math.PI * 0.1}
      />
    );
  }
);

CameraController.displayName = 'CameraController';
