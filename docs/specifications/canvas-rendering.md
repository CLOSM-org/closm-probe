# Canvas 2D Rendering Specification

Technical details for 3D projection and rendering of the storage universe using Canvas 2D API.

For conceptual background, see [Product Design - Part I: The Universe Metaphor](../design/product-design.md#part-i-the-universe-metaphor).

---

## 3D Projection

### Coordinate System

- X-axis: Left (-) to Right (+)
- Y-axis: Down (-) to Up (+)
- Z-axis: Away (-) to Toward (+)

### Projection Formula

```typescript
const project = (x: number, y: number, z: number, rotation: { x: number; y: number }, scale: number, center: { x: number; y: number }) => {
  const cosX = Math.cos(rotation.x);
  const sinX = Math.sin(rotation.x);
  const cosY = Math.cos(rotation.y);
  const sinY = Math.sin(rotation.y);

  // Y-axis rotation
  const x1 = x * cosY - z * sinY;
  const z1 = x * sinY + z * cosY;

  // X-axis rotation
  const y1 = y * cosX - z1 * sinX;
  const z2 = y * sinX + z1 * cosX;

  // Perspective projection
  const perspective = 8;
  const factor = perspective / (perspective + z2);

  return {
    screenX: center.x + x1 * scale * factor,
    screenY: center.y - y1 * scale * factor,
    depth: z2,
    scale: factor
  };
};
```

---

## Polar Coordinate System

Celestial bodies are positioned using polar coordinates mapped to temporal attributes.

### Orbital Radius (Creation Date)

Newer files orbit closer to the sun, older files drift outward:

```typescript
const calculateOrbitRadius = (createdAt: number, baseRadius: number) => {
  const ageInDays = (Date.now() - createdAt) / (1000 * 60 * 60 * 24);
  const ageFactor = Math.log10(Math.max(1, ageInDays)) / 3; // 0-1 range for 1-1000 days
  return baseRadius * (0.5 + ageFactor * 1.5); // 50% to 200% of base radius
};
```

### Angular Position (Last Modified)

Recently updated files cluster in the "hot zone" (top of view):

```typescript
const calculateAngle = (lastModified: number) => {
  const ageInDays = (Date.now() - lastModified) / (1000 * 60 * 60 * 24);
  const normalizedAge = Math.min(1, ageInDays / 365); // 0-1 over a year
  return -Math.PI / 2 + normalizedAge * Math.PI * 2; // Start from top, spread clockwise
};
```

### Converting Polar to Cartesian

```typescript
const polarToCartesian = (radius: number, angle: number, parentPos: {x: number, z: number}) => {
  return {
    x: parentPos.x + Math.cos(angle) * radius,
    z: parentPos.z + Math.sin(angle) * radius,
    y: 0 // Flat orbital plane
  };
};
```

---

## Celestial Body Rendering

### Size Calculation (Logarithmic Scale)

Planets (directories) and satellites (files) use logarithmic scaling to handle vastly different sizes.

```typescript
const calculateBodyRadius = (size: number, type: 'planet' | 'satellite') => {
  const baseSize = type === 'planet' ? 15 : 8;
  const scaleFactor = type === 'planet' ? 3 : 2;
  const minSize = type === 'planet' ? 1000 : 100;

  return baseSize + Math.log10(Math.max(size, minSize)) * scaleFactor;
};
```

### Safety Checks

**IMPORTANT**: Always validate radius values before using in Canvas API.

```typescript
// Safe radius for createRadialGradient
const safeRadius = Math.max(1, calculatedRadius);

// Safe ellipse parameters (for orbital paths)
ctx.ellipse(x, y, Math.max(1, radiusX), Math.max(0.5, radiusY), rotation, 0, Math.PI * 2);
```

### Z-Sorting

Celestial bodies must be sorted by depth before rendering (back to front):

```typescript
const sortedBodies = bodies
  .map(body => ({ ...body, projected: project(body.x, body.y, body.z) }))
  .sort((a, b) => a.projected.depth - b.projected.depth);
```

---

## Gradient Effects

### Planet Gradient (Directory)

Planets have a 3D spherical appearance with lighting from upper-left:

```typescript
const planetGradient = ctx.createRadialGradient(
  x - radius * 0.3, y - radius * 0.3, 0,  // Light source offset
  x, y, radius
);
planetGradient.addColorStop(0, color);
planetGradient.addColorStop(0.7, shadeColor(color, -20));
planetGradient.addColorStop(1, shadeColor(color, -40));
```

### Satellite Gradient (File)

Satellites have a highlight to distinguish from planets:

```typescript
const satGradient = ctx.createRadialGradient(
  x - radius * 0.3, y - radius * 0.3, 0,
  x, y, radius
);
satGradient.addColorStop(0, '#fff');      // Highlight
satGradient.addColorStop(0.3, color);
satGradient.addColorStop(1, shadeColor(color, -30));
```

### Glow Effect (Selection/Hover)

Selected or hovered celestial bodies emit a glow:

```typescript
const glowRadius = Math.max(1, radius * 3);
const glow = ctx.createRadialGradient(x, y, 0, x, y, glowRadius);
glow.addColorStop(0, `${color}66`);
glow.addColorStop(0.5, `${color}22`);
glow.addColorStop(1, 'transparent');
```

---

## Animation

### Frame Rate

```typescript
useEffect(() => {
  const interval = setInterval(() => {
    setTime(t => t + 0.02);
  }, 50);  // 20fps for smooth orbital motion
  return () => clearInterval(interval);
}, []);
```

### Pulse Effect

Planets (directories) pulse gently to indicate they can be explored:

```typescript
// Planet pulse animation
const pulse = body.type === 'planet'
  ? 0.9 + Math.sin(time * 1.5) * 0.1
  : 1;

const animatedRadius = baseRadius * pulse;
```

### Brightness by Age

Recent celestial bodies are brighter (see Visual Encoding in product design):

```typescript
const calculateBrightness = (lastModified: number) => {
  const age = (Date.now() - lastModified) / (1000 * 60 * 60 * 24 * 30); // months
  return Math.max(0.25, 1 - age * 0.1);
};
```

---

## Interaction

### Hit Detection

Detect which celestial body is under the cursor:

```typescript
const getBodyAt = (mouseX: number, mouseY: number) => {
  // Iterate in reverse (front to back for proper occlusion)
  for (const body of [...bodies].reverse()) {
    const projected = project(body.x, body.y, body.z);
    const radius = calculateBodyRadius(body.size, body.type) * projected.scale;

    const dist = Math.sqrt(
      (mouseX - projected.screenX) ** 2 +
      (mouseY - projected.screenY) ** 2
    );

    if (dist < radius + 5) {  // 5px tolerance for easier selection
      return body;
    }
  }
  return null;
};
```

### Drag Rotation (Orbit View)

Rotate the view around the sun (current root):

```typescript
const handleMouseMove = (e: MouseEvent) => {
  if (!isDragging) return;

  const dx = e.clientX - lastMouse.x;
  const dy = e.clientY - lastMouse.y;

  setRotation(prev => ({
    x: Math.max(-Math.PI / 2, Math.min(Math.PI / 2, prev.x + dy * 0.008)),
    y: prev.y + dx * 0.008
  }));

  setLastMouse({ x: e.clientX, y: e.clientY });
};
```

### Zoom (Approach/Retreat)

Get closer to or farther from the solar system:

```typescript
const handleWheel = (e: WheelEvent) => {
  e.preventDefault();
  setZoom(prev => Math.max(0.5, Math.min(2, prev - e.deltaY * 0.001)));
};
```

---

## Performance Considerations

### Large Universes

When visualizing large storage (thousands of celestial bodies):

- **Level of Detail (LOD)**: Reduce rendering detail for distant bodies
- **Web Workers**: Offload position calculations to background threads
- **Virtualization**: Don't render off-screen celestial bodies
- **Batch Operations**: Group canvas draw calls

### Memory Management

- Clear references in useEffect cleanup
- Use `useMemo` for expensive calculations (flattening, sorting)
- Avoid creating new objects in render loops

---

## Related Documentation

- [Product Design - Visual Encoding System](../design/product-design.md#3-visual-encoding-system)
- [Product Design - Rendering Architecture](../design/product-design.md#9-rendering-architecture)
