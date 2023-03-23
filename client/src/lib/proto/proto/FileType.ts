// Original file: ../proto/files.proto

export const FileType = {
  DOCUMENT: 'DOCUMENT',
} as const;

export type FileType =
  | 'DOCUMENT'
  | 0

export type FileType__Output = typeof FileType[keyof typeof FileType]
