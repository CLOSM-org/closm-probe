# Canvas 2D Rendering Specification

Technical details for 3D projection and rendering using Canvas 2D API.

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

## Node Rendering

### Size Calculation (Logarithmic Scale)

```typescript
const calculateNodeRadius = (size: number, type: 'file' | 'directory') => {
  const baseSize = type === 'directory' ? 15 : 8;
  const scaleFactor = type === 'directory' ? 3 : 2;
  const minSize = type === 'directory' ? 1000 : 100;

  return baseSize + Math.log10(Math.max(size, minSize)) * scaleFactor;
};
```

### Safety Checks

**IMPORTANT**: Always validate radius values before using in Canvas API.

```typescript
// Safe radius for createRadialGradient
const safeRadius = Math.max(1, calculatedRadius);

// Safe ellipse parameters
ctx.ellipse(x, y, Math.max(1, radiusX), Math.max(0.5, radiusY), rotation, 0, Math.PI * 2);
```

### Z-Sorting

Items must be sorted by depth before rendering:

```typescript
const sortedItems = items
  .map(item => ({ ...item, projected: project(item.x, item.y, item.z) }))
  .sort((a, b) => a.projected.depth - b.projected.depth);
```

---

## Gradient Effects

### Planet Gradient (Directory)

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
  }, 50);  // 20fps
  return () => clearInterval(interval);
}, []);
```

### Pulse Effect

```typescript
// Directory pulse
const pulse = item.type === 'directory'
  ? 0.9 + Math.sin(time * 1.5) * 0.1
  : 1;

const animatedRadius = baseRadius * pulse;
```

### Brightness by Age

```typescript
const calculateBrightness = (lastModified: number) => {
  const age = (Date.now() - lastModified) / (1000 * 60 * 60 * 24 * 30); // months
  return Math.max(0.5, 1 - age * 0.1);
};
```

---

## Interaction

### Hit Detection

```typescript
const getItemAt = (mouseX: number, mouseY: number) => {
  // Iterate in reverse (front to back)
  for (const item of [...items].reverse()) {
    const projected = project(item.x, item.y, item.z);
    const radius = calculateNodeRadius(item.size, item.type) * projected.scale;

    const dist = Math.sqrt(
      (mouseX - projected.screenX) ** 2 +
      (mouseY - projected.screenY) ** 2
    );

    if (dist < radius + 5) {  // 5px tolerance
      return item;
    }
  }
  return null;
};
```

### Drag Rotation

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

### Zoom

```typescript
const handleWheel = (e: WheelEvent) => {
  e.preventDefault();
  setZoom(prev => Math.max(0.5, Math.min(2, prev - e.deltaY * 0.001)));
};
```

---

## Performance Considerations

### Large Datasets

- Implement Level of Detail (LOD)
- Use Web Workers for position calculations
- Virtualize off-screen nodes
- Batch canvas operations

### Memory Management

- Clear references in useEffect cleanup
- Use `useMemo` for expensive calculations
- Avoid creating objects in render loops
