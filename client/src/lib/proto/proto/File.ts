// Original file: utils.proto

import type { FileType as _proto_FileType, FileType__Output as _proto_FileType__Output } from '../proto/FileType';

export interface File {
  'id'?: (string);
  'created'?: (string);
  'updated'?: (string);
  'deleted'?: (string);
  'targetId'?: (string);
  'fileName'?: (string);
  'fileSize'?: (string);
  'fileType'?: (_proto_FileType);
  'fileBuffer'?: (Buffer | Uint8Array | string);
}

export interface File__Output {
  'id': (string);
  'created': (string);
  'updated': (string);
  'deleted': (string);
  'targetId': (string);
  'fileName': (string);
  'fileSize': (string);
  'fileType': (_proto_FileType__Output);
  'fileBuffer': (Buffer);
}
