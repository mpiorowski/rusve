// Original file: main.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { Empty as _proto_Empty, Empty__Output as _proto_Empty__Output } from '../proto/Empty';
import type { File as _proto_File, File__Output as _proto_File__Output } from '../proto/File';
import type { Id as _proto_Id, Id__Output as _proto_Id__Output } from '../proto/Id';

export interface UtilsServiceClient extends grpc.Client {
  DeleteFile(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  DeleteFile(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  DeleteFile(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  DeleteFile(argument: _proto_Id, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteFile(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteFile(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteFile(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteFile(argument: _proto_Id, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  
  GetFile(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  GetFile(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  GetFile(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  GetFile(argument: _proto_Id, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFile(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFile(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFile(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFile(argument: _proto_Id, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  
  GetFiles(argument: _proto_Id, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  GetFiles(argument: _proto_Id, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  getFiles(argument: _proto_Id, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  getFiles(argument: _proto_Id, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  
  UploadFile(argument: _proto_File, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  UploadFile(argument: _proto_File, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  UploadFile(argument: _proto_File, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  UploadFile(argument: _proto_File, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  uploadFile(argument: _proto_File, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  uploadFile(argument: _proto_File, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  uploadFile(argument: _proto_File, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  uploadFile(argument: _proto_File, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  
}

export interface UtilsServiceHandlers extends grpc.UntypedServiceImplementation {
  DeleteFile: grpc.handleUnaryCall<_proto_Id__Output, _proto_Empty>;
  
  GetFile: grpc.handleUnaryCall<_proto_Id__Output, _proto_File>;
  
  GetFiles: grpc.handleServerStreamingCall<_proto_Id__Output, _proto_File>;
  
  UploadFile: grpc.handleUnaryCall<_proto_File__Output, _proto_File>;
  
}

export interface UtilsServiceDefinition extends grpc.ServiceDefinition {
  DeleteFile: MethodDefinition<_proto_Id, _proto_Empty, _proto_Id__Output, _proto_Empty__Output>
  GetFile: MethodDefinition<_proto_Id, _proto_File, _proto_Id__Output, _proto_File__Output>
  GetFiles: MethodDefinition<_proto_Id, _proto_File, _proto_Id__Output, _proto_File__Output>
  UploadFile: MethodDefinition<_proto_File, _proto_File, _proto_File__Output, _proto_File__Output>
}
