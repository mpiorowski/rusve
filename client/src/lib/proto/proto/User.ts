// Original file: users.proto

import type { UserRole as _proto_UserRole, UserRole__Output as _proto_UserRole__Output } from '../proto/UserRole';

export interface User {
  'id'?: (string);
  'created'?: (string);
  'updated'?: (string);
  'deleted'?: (string);
  'email'?: (string);
  'sub'?: (string);
  'role'?: (_proto_UserRole);
  'avatar'?: (string);
  'subscriptionId'?: (string);
  'subscriptionEnd'?: (string);
  'subscriptionCheck'?: (string);
  'subscriptionActive'?: (boolean);
}

export interface User__Output {
  'id': (string);
  'created': (string);
  'updated': (string);
  'deleted': (string);
  'email': (string);
  'sub': (string);
  'role': (_proto_UserRole__Output);
  'avatar': (string);
  'subscriptionId': (string);
  'subscriptionEnd': (string);
  'subscriptionCheck': (string);
  'subscriptionActive': (boolean);
}
