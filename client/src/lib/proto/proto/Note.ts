// Original file: ../proto/notes.proto

import type { User as _proto_User, User__Output as _proto_User__Output } from '../proto/User';

export interface Note {
  'id'?: (Buffer | Uint8Array | string);
  'created'?: (string);
  'updated'?: (string);
  'deleted'?: (string);
  'userId'?: (Buffer | Uint8Array | string);
  'title'?: (string);
  'content'?: (string);
  'user'?: (_proto_User | null);
  '_deleted'?: "deleted";
}

export interface Note__Output {
  'id': (Buffer);
  'created': (string);
  'updated': (string);
  'deleted'?: (string);
  'userId': (Buffer);
  'title': (string);
  'content': (string);
  'user': (_proto_User__Output | null);
  '_deleted': "deleted";
}
