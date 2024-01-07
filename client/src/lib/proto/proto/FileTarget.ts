// Original file: utils.proto

export const FileTarget = {
  FILE_UNSET: 0,
  DOCUMENT: 1,
  AVATAR: 2,
} as const;

export type FileTarget =
  | 'FILE_UNSET'
  | 0
  | 'DOCUMENT'
  | 1
  | 'AVATAR'
  | 2

export type FileTarget__Output = typeof FileTarget[keyof typeof FileTarget]
