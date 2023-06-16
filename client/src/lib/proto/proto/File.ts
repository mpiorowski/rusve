// Original file: ../proto/utils.proto

import type { FileType as _proto_FileType, FileType__Output as _proto_FileType__Output } from '../proto/FileType';

export interface File {
  'id'?: (Buffer | Uint8Array | string);
  'created'?: (string);
  'updated'?: (string);
  'deleted'?: (string);
  'targetId'?: (Buffer | Uint8Array | string);
  'name'?: (string);
  'type'?: (_proto_FileType);
  'buffer'?: (Buffer | Uint8Array | string);
  'url'?: (string);
  '_deleted'?: "deleted";
}

export interface File__Output {
  'id': (Buffer);
  'created': (string);
  'updated': (string);
  'deleted'?: (string);
  'targetId': (Buffer);
  'name': (string);
  'type': (_proto_FileType__Output);
  'buffer': (Buffer);
  'url': (string);
  '_deleted': "deleted";
}
