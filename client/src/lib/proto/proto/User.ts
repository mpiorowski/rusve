// Original file: ../proto/users.proto

import type { UserRole as _proto_UserRole, UserRole__Output as _proto_UserRole__Output } from '../proto/UserRole';

export interface User {
  'id'?: (Buffer | Uint8Array | string);
  'created'?: (string);
  'updated'?: (string);
  'deleted'?: (string);
  'email'?: (string);
  'role'?: (_proto_UserRole);
  'sub'?: (string);
  'name'?: (string);
  'avatarId'?: (Buffer | Uint8Array | string);
  'paymentId'?: (string);
  '_deleted'?: "deleted";
  '_avatarId'?: "avatarId";
  '_paymentId'?: "paymentId";
}

export interface User__Output {
  'id': (Buffer);
  'created': (string);
  'updated': (string);
  'deleted'?: (string);
  'email': (string);
  'role': (_proto_UserRole__Output);
  'sub': (string);
  'name': (string);
  'avatarId'?: (Buffer);
  'paymentId'?: (string);
  '_deleted': "deleted";
  '_avatarId': "avatarId";
  '_paymentId': "paymentId";
}
