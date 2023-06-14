// Original file: ../proto/notes.proto


export interface Note {
  'id'?: (Buffer | Uint8Array | string);
  'created'?: (string);
  'updated'?: (string);
  'deleted'?: (string);
  'userId'?: (Buffer | Uint8Array | string);
  'title'?: (string);
  'content'?: (string);
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
  '_deleted': "deleted";
}
