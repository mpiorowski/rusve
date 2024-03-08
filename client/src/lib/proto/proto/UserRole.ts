// Original file: users.proto

export const UserRole = {
  UNSET: 0,
  USER: 1,
  ADMIN: 2,
} as const;

export type UserRole =
  | 'UNSET'
  | 0
  | 'USER'
  | 1
  | 'ADMIN'
  | 2

export type UserRole__Output = typeof UserRole[keyof typeof UserRole]
