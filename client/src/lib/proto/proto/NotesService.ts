// Original file: main.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { Empty as _proto_Empty, Empty__Output as _proto_Empty__Output } from '../proto/Empty';
import type { Id as _proto_Id, Id__Output as _proto_Id__Output } from '../proto/Id';
import type { Note as _proto_Note, Note__Output as _proto_Note__Output } from '../proto/Note';

export interface NotesServiceClient extends grpc.Client {
  CreateNote(argument: _proto_Note, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  CreateNote(argument: _proto_Note, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  CreateNote(argument: _proto_Note, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  CreateNote(argument: _proto_Note, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  createNote(argument: _proto_Note, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  createNote(argument: _proto_Note, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  createNote(argument: _proto_Note, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  createNote(argument: _proto_Note, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  
  DeleteNoteById(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  DeleteNoteById(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  DeleteNoteById(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  DeleteNoteById(argument: _proto_Id, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteNoteById(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteNoteById(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteNoteById(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteNoteById(argument: _proto_Id, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  
  GetNoteById(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  GetNoteById(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  GetNoteById(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  GetNoteById(argument: _proto_Id, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  getNoteById(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  getNoteById(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  getNoteById(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  getNoteById(argument: _proto_Id, callback: grpc.requestCallback<_proto_Note__Output>): grpc.ClientUnaryCall;
  
  GetNotesByUserId(argument: _proto_Empty, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Note__Output>;
  GetNotesByUserId(argument: _proto_Empty, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Note__Output>;
  getNotesByUserId(argument: _proto_Empty, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Note__Output>;
  getNotesByUserId(argument: _proto_Empty, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Note__Output>;
  
}

export interface NotesServiceHandlers extends grpc.UntypedServiceImplementation {
  CreateNote: grpc.handleUnaryCall<_proto_Note__Output, _proto_Note>;
  
  DeleteNoteById: grpc.handleUnaryCall<_proto_Id__Output, _proto_Empty>;
  
  GetNoteById: grpc.handleUnaryCall<_proto_Id__Output, _proto_Note>;
  
  GetNotesByUserId: grpc.handleServerStreamingCall<_proto_Empty__Output, _proto_Note>;
  
}

export interface NotesServiceDefinition extends grpc.ServiceDefinition {
  CreateNote: MethodDefinition<_proto_Note, _proto_Note, _proto_Note__Output, _proto_Note__Output>
  DeleteNoteById: MethodDefinition<_proto_Id, _proto_Empty, _proto_Id__Output, _proto_Empty__Output>
  GetNoteById: MethodDefinition<_proto_Id, _proto_Note, _proto_Id__Output, _proto_Note__Output>
  GetNotesByUserId: MethodDefinition<_proto_Empty, _proto_Note, _proto_Empty__Output, _proto_Note__Output>
}
