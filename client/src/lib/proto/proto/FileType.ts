// Original file: utils.proto

export const FileType = {
  FILE_UNSET: 0,
  FILE_DOCUMENT: 1,
  FILE_AVATAR: 2,
} as const;

export type FileType =
  | 'FILE_UNSET'
  | 0
  | 'FILE_DOCUMENT'
  | 1
  | 'FILE_AVATAR'
  | 2

export type FileType__Output = typeof FileType[keyof typeof FileType]
