// Original file: main.proto

import type { Note as _proto_Note, Note__Output as _proto_Note__Output } from '../proto/Note';
import type { Profile as _proto_Profile, Profile__Output as _proto_Profile__Output } from '../proto/Profile';

export interface NoteResponse {
  'note'?: (_proto_Note | null);
  'profile'?: (_proto_Profile | null);
}

export interface NoteResponse__Output {
  'note': (_proto_Note__Output | null);
  'profile': (_proto_Profile__Output | null);
}
