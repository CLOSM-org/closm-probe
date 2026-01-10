// Shared types for Universe components

export interface FileNode {
  name: string;
  type: 'file' | 'directory';
  size: number;
  fileType?: string;
  createdAt?: number;      // Creation timestamp for orbital radius
  lastModified?: number;   // Last modified timestamp for angular position
  children?: FileNode[];
}

export interface PositionedItem extends FileNode {
  path: string;
  depth: number;
  x: number;
  y: number;
  z: number;
  angle?: number;
  orbitRadius: number;
  parentPos?: { x: number; y: number; z: number };
}

// Asteroid belt: represents a group of small files consolidated into a ring
export interface AsteroidBelt {
  id: string;
  parentPath: string;
  parentPos: { x: number; y: number; z: number };
  orbitRadius: number;
  files: FileNode[];
  count: number;
  totalSize: number;
}

// Maximum number of planets displayed (overflow goes to asteroid belt)
export const MAX_PLANETS = 20;

// Separate children into planets (max 20) and asteroids (overflow)
// Items are sorted by size (largest first) to determine which become planets
export function classifyChildren(children: FileNode[]): {
  significantItems: FileNode[];
  asteroids: FileNode[];
} {
  // Sort by size descending (largest first)
  const sorted = [...children].sort((a, b) => b.size - a.size);

  // Top MAX_PLANETS become planets, rest become asteroids
  const significantItems = sorted.slice(0, MAX_PLANETS);
  const asteroids = sorted.slice(MAX_PLANETS);

  return { significantItems, asteroids };
}

// File type colors
export const typeColors: Record<string, string> = {
  code: '#61dafb',
  design: '#a855f7',
  image: '#f59e0b',
  video: '#ef4444',
  pdf: '#dc2626',
  doc: '#3b82f6',
  text: '#22c55e',
  data: '#06b6d4',
  archive: '#6b7280',
  directory: '#ffffff', // White for directories (star/planet)
};

// SIZE_UNKNOWN marker for items with pending size calculation
export const SIZE_UNKNOWN = -1;

// Helper: format bytes to human readable
export function formatSize(bytes: number): string {
  // Handle unknown size (calculating)
  if (bytes === SIZE_UNKNOWN) return '計算中...';
  if (bytes < 0) return '---';
  if (bytes < 1024) return bytes + ' B';
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
  if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
  return (bytes / (1024 * 1024 * 1024)).toFixed(2) + ' GB';
}

// Helper: format time ago
export function formatTimeAgo(timestamp: number): string {
  const diff = Date.now() - timestamp;
  const hours = Math.floor(diff / 3600000);
  if (hours < 1) return '1時間以内';
  if (hours < 24) return `${hours}時間前`;
  const days = Math.floor(hours / 24);
  if (days < 7) return `${days}日前`;
  const weeks = Math.floor(days / 7);
  if (weeks < 4) return `${weeks}週間前`;
  return `${Math.floor(days / 30)}ヶ月前`;
}

// Size constants for radius calculation (enhanced contrast)
const SIZE_LIMITS = {
  file: {
    minRadius: 0.15,  // Very small files barely visible
    maxRadius: 2.5,   // Large files prominent
    minLog: 2,        // 100B
    maxLog: 9,        // 1GB
  },
  directory: {
    minRadius: 0.25,  // Small directories compact
    maxRadius: 3.0,   // Large directories prominent
    minLog: 3,        // 1KB
    maxLog: 10,       // 10GB
  },
};

// Star (center) fixed radius - always largest
export const STAR_RADIUS = 4.0;

// Helper: calculate node radius based on size (log scale with min/max interpolation)
// Creates ~6x difference between smallest and largest files
export function calculateNodeRadius(size: number, type: 'file' | 'directory'): number {
  const limits = SIZE_LIMITS[type];
  const logSize = Math.log10(Math.max(size, 1));

  // Clamp to range and interpolate
  const normalizedLog = Math.max(0, Math.min(1,
    (logSize - limits.minLog) / (limits.maxLog - limits.minLog)
  ));

  return limits.minRadius + normalizedLog * (limits.maxRadius - limits.minRadius);
}

// Planet radius limits for relative sizing
export const PLANET_RADIUS = {
  min: 0.4,
  max: 2.5,
};

// Helper: calculate planet radius relative to displayed items
// Ensures clear visual difference between smallest and largest items
export function calculateRelativeRadius(
  size: number,
  minSize: number,
  maxSize: number
): number {
  // If all items have same size, return middle value
  if (maxSize === minSize) {
    return (PLANET_RADIUS.min + PLANET_RADIUS.max) / 2;
  }

  // Linear interpolation between min and max radius based on relative size
  const ratio = (size - minSize) / (maxSize - minSize);
  return PLANET_RADIUS.min + ratio * (PLANET_RADIUS.max - PLANET_RADIUS.min);
}

// Helper: calculate brightness based on age
export function calculateBrightness(lastModified?: number): number {
  if (!lastModified) return 0.7;
  const age = (Date.now() - lastModified) / (1000 * 60 * 60 * 24 * 30); // months
  return Math.max(0.3, 1 - age * 0.1);
}

// Helper: calculate equal-spaced angle with sorted index
// Items are sorted by lastModified, then placed with equal spacing starting from 12 o'clock
export function calculateEqualSpacedAngle(sortedIndex: number, totalItems: number): number {
  if (totalItems <= 0) return 0;
  // Start from 12 o'clock (-π/2) and go clockwise
  return -Math.PI / 2 + (sortedIndex / totalItems) * Math.PI * 2;
}

// Helper: calculate orbital radius based on creation order
// Newer items (by creation) get inner orbits, older get outer orbits
export function calculateOrbitRadiusByOrder(creationOrderIndex: number, totalItems: number, baseRadius: number): number {
  if (totalItems <= 1) return baseRadius;
  // Range from 60% to 140% of base radius based on creation order
  const factor = 0.6 + (creationOrderIndex / (totalItems - 1)) * 0.8;
  return baseRadius * factor;
}

// Helper: sort children by last modified (newest first) and return sorted array with indices
export function sortByLastModified<T extends { lastModified?: number }>(items: T[]): T[] {
  return [...items].sort((a, b) => (b.lastModified || 0) - (a.lastModified || 0));
}

// Helper: sort children by creation date (newest first) and return sorted array
export function sortByCreatedAt<T extends { createdAt?: number }>(items: T[]): T[] {
  return [...items].sort((a, b) => (b.createdAt || 0) - (a.createdAt || 0));
}
