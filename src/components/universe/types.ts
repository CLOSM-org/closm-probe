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
  directory: '#8b5cf6',
};

// Helper: format bytes to human readable
export function formatSize(bytes: number): string {
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

// Helper: calculate node radius (log scale)
export function calculateNodeRadius(size: number, type: 'file' | 'directory'): number {
  const baseSize = type === 'directory' ? 0.5 : 0.25;
  const scaleFactor = type === 'directory' ? 0.1 : 0.05;
  const minSize = type === 'directory' ? 1000 : 100;
  return baseSize + Math.log10(Math.max(size, minSize)) * scaleFactor;
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
