// Original file: ../proto/utils.proto

export const FileType = {
  DOCUMENT: 'DOCUMENT',
  AVATAR: 'AVATAR',
} as const;

export type FileType =
  | 'DOCUMENT'
  | 0
  | 'AVATAR'
  | 1

export type FileType__Output = typeof FileType[keyof typeof FileType]
