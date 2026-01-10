'use server';

import * as fs from 'fs/promises';
import * as path from 'path';
import { exec } from 'child_process';
import { promisify } from 'util';
import { FileNode, SIZE_UNKNOWN } from '@/components/universe/types';

const execAsync = promisify(exec);

// File type detection based on extension
function detectFileType(filename: string): string {
  const ext = path.extname(filename).toLowerCase();

  const typeMap: Record<string, string> = {
    // Code
    '.ts': 'code', '.tsx': 'code', '.js': 'code', '.jsx': 'code',
    '.py': 'code', '.rb': 'code', '.go': 'code', '.rs': 'code',
    '.java': 'code', '.c': 'code', '.cpp': 'code', '.h': 'code',
    '.css': 'code', '.scss': 'code', '.html': 'code', '.vue': 'code',
    '.swift': 'code', '.kt': 'code', '.php': 'code', '.sh': 'code',

    // Design
    '.psd': 'design', '.ai': 'design', '.sketch': 'design',
    '.fig': 'design', '.xd': 'design', '.svg': 'design',

    // Images
    '.png': 'image', '.jpg': 'image', '.jpeg': 'image', '.gif': 'image',
    '.webp': 'image', '.ico': 'image', '.bmp': 'image', '.tiff': 'image',

    // Video
    '.mp4': 'video', '.mov': 'video', '.avi': 'video', '.mkv': 'video',
    '.webm': 'video', '.flv': 'video', '.wmv': 'video',

    // PDF
    '.pdf': 'pdf',

    // Documents
    '.doc': 'doc', '.docx': 'doc', '.ppt': 'doc', '.pptx': 'doc',
    '.xls': 'doc', '.xlsx': 'doc', '.odt': 'doc', '.ods': 'doc',

    // Text
    '.txt': 'text', '.md': 'text', '.markdown': 'text', '.rst': 'text',
    '.json': 'text', '.yaml': 'text', '.yml': 'text', '.xml': 'text',
    '.csv': 'text', '.log': 'text', '.ini': 'text', '.toml': 'text',

    // Data
    '.db': 'data', '.sqlite': 'data', '.sql': 'data',
    '.sqlite3': 'data', '.mdb': 'data',

    // Archive
    '.zip': 'archive', '.tar': 'archive', '.gz': 'archive',
    '.rar': 'archive', '.7z': 'archive', '.bz2': 'archive',
  };

  return typeMap[ext] || 'text';
}

// Hidden/system files to skip
const SKIP_PATTERNS = [
  '.DS_Store',
  'Thumbs.db',
  '.git',
  'node_modules',
  '.next',
  '__pycache__',
  '.pytest_cache',
  '.venv',
  'venv',
  '.env',
  '.cache',
];

function shouldSkip(name: string): boolean {
  return SKIP_PATTERNS.includes(name) || name.startsWith('.');
}

export interface ReadDirectoryResult {
  success: boolean;
  data?: FileNode;
  error?: string;
}

export interface ListRootsResult {
  success: boolean;
  roots?: { path: string; name: string }[];
  error?: string;
}

// Read a directory and return FileNode structure
// depth: how many levels deep to read (0 = only immediate children)
export async function readDirectory(
  dirPath: string,
  depth: number = 2,
  includeHidden: boolean = false
): Promise<ReadDirectoryResult> {
  try {
    // Resolve and validate path
    const resolvedPath = path.resolve(dirPath);

    // Check if path exists and is a directory
    const stats = await fs.stat(resolvedPath);
    if (!stats.isDirectory()) {
      return { success: false, error: 'Path is not a directory' };
    }

    const node = await readNode(resolvedPath, depth, includeHidden);
    return { success: true, data: node };
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Unknown error';
    return { success: false, error: message };
  }
}

// Recursive function to read a node (file or directory)
async function readNode(
  nodePath: string,
  remainingDepth: number,
  includeHidden: boolean
): Promise<FileNode> {
  const stats = await fs.stat(nodePath);
  const name = path.basename(nodePath);

  const baseNode: FileNode = {
    name,
    type: stats.isDirectory() ? 'directory' : 'file',
    size: stats.size,
    createdAt: stats.birthtime.getTime(),
    lastModified: stats.mtime.getTime(),
  };

  if (stats.isFile()) {
    baseNode.fileType = detectFileType(name);
    return baseNode;
  }

  // It's a directory
  if (remainingDepth < 0) {
    // Don't recurse into children, but still calculate size from immediate children
    try {
      const entries = await fs.readdir(nodePath, { withFileTypes: true });
      let totalSize = 0;

      for (const entry of entries) {
        if (!includeHidden && shouldSkip(entry.name)) continue;

        const childPath = path.join(nodePath, entry.name);
        try {
          const childStats = await fs.stat(childPath);
          if (childStats.isFile()) {
            totalSize += childStats.size;
          }
          // For subdirectories, we don't recurse - their size is unknown
        } catch {
          // Skip unreadable files
        }
      }

      baseNode.size = totalSize;
      baseNode.children = []; // Mark as not expanded
    } catch {
      baseNode.size = 0;
      baseNode.children = [];
    }
    return baseNode;
  }

  try {
    const entries = await fs.readdir(nodePath, { withFileTypes: true });
    const children: FileNode[] = [];
    let totalSize = 0;

    for (const entry of entries) {
      // Skip hidden/system files unless requested
      if (!includeHidden && shouldSkip(entry.name)) {
        continue;
      }

      const childPath = path.join(nodePath, entry.name);

      try {
        const childNode = await readNode(childPath, remainingDepth - 1, includeHidden);
        children.push(childNode);
        totalSize += childNode.size;
      } catch {
        // Skip files we can't read (permissions, etc.)
        continue;
      }
    }

    baseNode.children = children;
    baseNode.size = totalSize;

    return baseNode;
  } catch {
    // Can't read directory contents
    baseNode.children = [];
    return baseNode;
  }
}

// Get common root directories for selection
export async function getAvailableRoots(): Promise<ListRootsResult> {
  try {
    const homeDir = process.env.HOME || process.env.USERPROFILE || '/';

    const potentialRoots = [
      { path: homeDir, name: 'Home' },
      { path: path.join(homeDir, 'Documents'), name: 'Documents' },
      { path: path.join(homeDir, 'Downloads'), name: 'Downloads' },
      { path: path.join(homeDir, 'Desktop'), name: 'Desktop' },
      { path: path.join(homeDir, 'Work'), name: 'Work' },
      { path: path.join(homeDir, 'Projects'), name: 'Projects' },
      { path: '/tmp', name: 'Temp' },
    ];

    // Filter to only existing directories
    const roots: { path: string; name: string }[] = [];

    for (const root of potentialRoots) {
      try {
        const stats = await fs.stat(root.path);
        if (stats.isDirectory()) {
          roots.push(root);
        }
      } catch {
        // Skip non-existent paths
      }
    }

    return { success: true, roots };
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Unknown error';
    return { success: false, error: message };
  }
}

// List children of a directory (shallow read for lazy loading)
export async function listChildren(
  dirPath: string,
  includeHidden: boolean = false
): Promise<ReadDirectoryResult> {
  return readDirectory(dirPath, 0, includeHidden);
}

// Expand a specific path (for caching/lazy loading)
export async function expandPath(
  dirPath: string,
  depth: number = 1,
  includeHidden: boolean = false
): Promise<ReadDirectoryResult> {
  return readDirectory(dirPath, depth, includeHidden);
}

// ============================================================
// NEW APIs: Structure-only loading + Async size calculation
// ============================================================

export interface SizeResult {
  success: boolean;
  size?: number;
  error?: string;
}

// Read directory structure only (fast, no recursive size calculation)
// Returns FileNode with size = SIZE_UNKNOWN (-1) for directories
// Used for fast navigation with unlimited depth
export async function readDirectoryStructure(
  dirPath: string,
  depth: number = 2,
  includeHidden: boolean = false
): Promise<ReadDirectoryResult> {
  try {
    const resolvedPath = path.resolve(dirPath);

    const stats = await fs.stat(resolvedPath);
    if (!stats.isDirectory()) {
      return { success: false, error: 'Path is not a directory' };
    }

    const node = await readNodeStructure(resolvedPath, depth, includeHidden);
    return { success: true, data: node };
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Unknown error';
    return { success: false, error: message };
  }
}

// Internal: Read node structure without recursive size calculation
async function readNodeStructure(
  nodePath: string,
  remainingDepth: number,
  includeHidden: boolean
): Promise<FileNode> {
  const stats = await fs.stat(nodePath);
  const name = path.basename(nodePath);

  const baseNode: FileNode = {
    name,
    type: stats.isDirectory() ? 'directory' : 'file',
    size: stats.isFile() ? stats.size : SIZE_UNKNOWN, // Files get real size, directories get -1
    createdAt: stats.birthtime.getTime(),
    lastModified: stats.mtime.getTime(),
  };

  if (stats.isFile()) {
    baseNode.fileType = detectFileType(name);
    return baseNode;
  }

  // It's a directory
  if (remainingDepth < 0) {
    // Don't read children at all
    baseNode.children = [];
    return baseNode;
  }

  try {
    const entries = await fs.readdir(nodePath, { withFileTypes: true });
    const children: FileNode[] = [];

    for (const entry of entries) {
      if (!includeHidden && shouldSkip(entry.name)) continue;

      const childPath = path.join(nodePath, entry.name);

      try {
        const childNode = await readNodeStructure(childPath, remainingDepth - 1, includeHidden);
        children.push(childNode);
      } catch {
        // Skip unreadable files
        continue;
      }
    }

    baseNode.children = children;
    return baseNode;
  } catch {
    baseNode.children = [];
    return baseNode;
  }
}

// Calculate directory size using native methods
// Uses 'du' command on macOS/Linux (fast), fallback to recursive stat on Windows
export async function getDirectorySize(dirPath: string): Promise<SizeResult> {
  try {
    const resolvedPath = path.resolve(dirPath);

    // Verify it's a directory
    const stats = await fs.stat(resolvedPath);
    if (!stats.isDirectory()) {
      return { success: false, error: 'Path is not a directory' };
    }

    // Try using 'du' command (fast on macOS/Linux)
    if (process.platform !== 'win32') {
      try {
        const { stdout } = await execAsync(`du -sk "${resolvedPath}" 2>/dev/null`);
        const sizeKB = parseInt(stdout.split('\t')[0], 10);
        if (!isNaN(sizeKB)) {
          return { success: true, size: sizeKB * 1024 };
        }
      } catch {
        // Fall through to recursive calculation
      }
    }

    // Fallback: recursive calculation (slower but cross-platform)
    const size = await calculateDirSizeRecursive(resolvedPath);
    return { success: true, size };
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Unknown error';
    return { success: false, error: message };
  }
}

// Recursive directory size calculation (fallback for Windows)
async function calculateDirSizeRecursive(dirPath: string): Promise<number> {
  let totalSize = 0;

  try {
    const entries = await fs.readdir(dirPath, { withFileTypes: true });

    for (const entry of entries) {
      // Skip system files
      if (shouldSkip(entry.name)) continue;

      const entryPath = path.join(dirPath, entry.name);

      try {
        if (entry.isFile()) {
          const stats = await fs.stat(entryPath);
          totalSize += stats.size;
        } else if (entry.isDirectory()) {
          totalSize += await calculateDirSizeRecursive(entryPath);
        }
      } catch {
        // Skip unreadable files
      }
    }
  } catch {
    // Return 0 if directory is unreadable
  }

  return totalSize;
}

// Calculate sizes for multiple directories in parallel
export async function getDirectorySizes(
  dirPaths: string[]
): Promise<{ [path: string]: SizeResult }> {
  const results: { [path: string]: SizeResult } = {};

  await Promise.all(
    dirPaths.map(async (dirPath) => {
      results[dirPath] = await getDirectorySize(dirPath);
    })
  );

  return results;
}
