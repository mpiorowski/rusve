// Original file: ../proto/posts.proto

import type { User as _proto_User, User__Output as _proto_User__Output } from '../proto/User';

export interface Post {
  'id'?: (string);
  'created'?: (string);
  'updated'?: (string);
  'deleted'?: (string);
  'userId'?: (string);
  'title'?: (string);
  'content'?: (string);
  'user'?: (_proto_User | null);
  '_deleted'?: "deleted";
}

export interface Post__Output {
  'id': (string);
  'created': (string);
  'updated': (string);
  'deleted'?: (string);
  'userId': (string);
  'title': (string);
  'content': (string);
  'user': (_proto_User__Output | null);
  '_deleted': "deleted";
}
