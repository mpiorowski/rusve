// Original file: ../proto/main.proto


export interface NoteId {
  'noteId'?: (Buffer | Uint8Array | string);
  'userId'?: (Buffer | Uint8Array | string);
}

export interface NoteId__Output {
  'noteId': (Buffer);
  'userId': (Buffer);
}
