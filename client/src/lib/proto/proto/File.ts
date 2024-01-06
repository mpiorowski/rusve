// Original file: utils.proto

import type { FileTarget as _proto_FileTarget, FileTarget__Output as _proto_FileTarget__Output } from '../proto/FileTarget';

export interface File {
  'id'?: (string);
  'created'?: (string);
  'updated'?: (string);
  'deleted'?: (string);
  'target_id'?: (string);
  'file_name'?: (string);
  'file_size'?: (string);
  'file_type'?: (string);
  'file_target'?: (_proto_FileTarget);
  'file_buffer'?: (Buffer | Uint8Array | string);
}

export interface File__Output {
  'id': (string);
  'created': (string);
  'updated': (string);
  'deleted': (string);
  'target_id': (string);
  'file_name': (string);
  'file_size': (string);
  'file_type': (string);
  'file_target': (_proto_FileTarget__Output);
  'file_buffer': (Buffer);
}
