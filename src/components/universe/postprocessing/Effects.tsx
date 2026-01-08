'use client';

import { EffectComposer, Bloom, Vignette } from '@react-three/postprocessing';
import { BlendFunction } from 'postprocessing';

interface EffectsProps {
  enableBloom?: boolean;
  enableVignette?: boolean;
}

export function Effects({ enableBloom = true, enableVignette = true }: EffectsProps) {
  return (
    <EffectComposer>
      {enableBloom && (
        <Bloom
          intensity={0.5}
          luminanceThreshold={0.2}
          luminanceSmoothing={0.9}
          mipmapBlur
        />
      )}
      {enableVignette && (
        <Vignette
          offset={0.3}
          darkness={0.6}
          blendFunction={BlendFunction.NORMAL}
        />
      )}
    </EffectComposer>
  );
}
