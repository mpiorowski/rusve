// Original file: utils.proto

export const FileType = {
  DOCUMENT: 0,
  AVATAR: 1,
} as const;

export type FileType =
  | 'DOCUMENT'
  | 0
  | 'AVATAR'
  | 1

export type FileType__Output = typeof FileType[keyof typeof FileType]
