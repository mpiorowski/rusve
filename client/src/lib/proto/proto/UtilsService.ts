// Original file: main.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { Count as _proto_Count, Count__Output as _proto_Count__Output } from '../proto/Count';
import type { Email as _proto_Email, Email__Output as _proto_Email__Output } from '../proto/Email';
import type { Empty as _proto_Empty, Empty__Output as _proto_Empty__Output } from '../proto/Empty';
import type { File as _proto_File, File__Output as _proto_File__Output } from '../proto/File';
import type { Id as _proto_Id, Id__Output as _proto_Id__Output } from '../proto/Id';
import type { Page as _proto_Page, Page__Output as _proto_Page__Output } from '../proto/Page';

export interface UtilsServiceClient extends grpc.Client {
  CountEmailsByTargetId(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  CountEmailsByTargetId(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  CountEmailsByTargetId(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  CountEmailsByTargetId(argument: _proto_Empty, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  countEmailsByTargetId(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  countEmailsByTargetId(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  countEmailsByTargetId(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  countEmailsByTargetId(argument: _proto_Empty, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  
  CountFilesByTargetId(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  CountFilesByTargetId(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  CountFilesByTargetId(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  CountFilesByTargetId(argument: _proto_Empty, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  countFilesByTargetId(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  countFilesByTargetId(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  countFilesByTargetId(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  countFilesByTargetId(argument: _proto_Empty, callback: grpc.requestCallback<_proto_Count__Output>): grpc.ClientUnaryCall;
  
  DeleteFileById(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  DeleteFileById(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  DeleteFileById(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  DeleteFileById(argument: _proto_Id, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteFileById(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteFileById(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteFileById(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  deleteFileById(argument: _proto_Id, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  
  GetEmailsByTargetId(argument: _proto_Page, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Email__Output>;
  GetEmailsByTargetId(argument: _proto_Page, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Email__Output>;
  getEmailsByTargetId(argument: _proto_Page, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Email__Output>;
  getEmailsByTargetId(argument: _proto_Page, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Email__Output>;
  
  GetFileById(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  GetFileById(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  GetFileById(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  GetFileById(argument: _proto_Id, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFileById(argument: _proto_Id, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFileById(argument: _proto_Id, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFileById(argument: _proto_Id, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFileById(argument: _proto_Id, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  
  GetFilesByTargetId(argument: _proto_Page, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  GetFilesByTargetId(argument: _proto_Page, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  getFilesByTargetId(argument: _proto_Page, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  getFilesByTargetId(argument: _proto_Page, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  
  SendEmail(argument: _proto_Email, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Email__Output>): grpc.ClientUnaryCall;
  SendEmail(argument: _proto_Email, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Email__Output>): grpc.ClientUnaryCall;
  SendEmail(argument: _proto_Email, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Email__Output>): grpc.ClientUnaryCall;
  SendEmail(argument: _proto_Email, callback: grpc.requestCallback<_proto_Email__Output>): grpc.ClientUnaryCall;
  sendEmail(argument: _proto_Email, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Email__Output>): grpc.ClientUnaryCall;
  sendEmail(argument: _proto_Email, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Email__Output>): grpc.ClientUnaryCall;
  sendEmail(argument: _proto_Email, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Email__Output>): grpc.ClientUnaryCall;
  sendEmail(argument: _proto_Email, callback: grpc.requestCallback<_proto_Email__Output>): grpc.ClientUnaryCall;
  
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
  CountEmailsByTargetId: grpc.handleUnaryCall<_proto_Empty__Output, _proto_Count>;
  
  CountFilesByTargetId: grpc.handleUnaryCall<_proto_Empty__Output, _proto_Count>;
  
  DeleteFileById: grpc.handleUnaryCall<_proto_Id__Output, _proto_Empty>;
  
  GetEmailsByTargetId: grpc.handleServerStreamingCall<_proto_Page__Output, _proto_Email>;
  
  GetFileById: grpc.handleUnaryCall<_proto_Id__Output, _proto_File>;
  
  GetFilesByTargetId: grpc.handleServerStreamingCall<_proto_Page__Output, _proto_File>;
  
  SendEmail: grpc.handleUnaryCall<_proto_Email__Output, _proto_Email>;
  
  UploadFile: grpc.handleUnaryCall<_proto_File__Output, _proto_File>;
  
}

export interface UtilsServiceDefinition extends grpc.ServiceDefinition {
  CountEmailsByTargetId: MethodDefinition<_proto_Empty, _proto_Count, _proto_Empty__Output, _proto_Count__Output>
  CountFilesByTargetId: MethodDefinition<_proto_Empty, _proto_Count, _proto_Empty__Output, _proto_Count__Output>
  DeleteFileById: MethodDefinition<_proto_Id, _proto_Empty, _proto_Id__Output, _proto_Empty__Output>
  GetEmailsByTargetId: MethodDefinition<_proto_Page, _proto_Email, _proto_Page__Output, _proto_Email__Output>
  GetFileById: MethodDefinition<_proto_Id, _proto_File, _proto_Id__Output, _proto_File__Output>
  GetFilesByTargetId: MethodDefinition<_proto_Page, _proto_File, _proto_Page__Output, _proto_File__Output>
  SendEmail: MethodDefinition<_proto_Email, _proto_Email, _proto_Email__Output, _proto_Email__Output>
  UploadFile: MethodDefinition<_proto_File, _proto_File, _proto_File__Output, _proto_File__Output>
}
