// Original file: ../proto/main.proto


export interface FileId {
  'fileId'?: (Buffer | Uint8Array | string);
  'targetId'?: (Buffer | Uint8Array | string);
}

export interface FileId__Output {
  'fileId': (Buffer);
  'targetId': (Buffer);
}
