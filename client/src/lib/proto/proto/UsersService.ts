// Original file: ../proto/main.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { AuthRequest as _proto_AuthRequest, AuthRequest__Output as _proto_AuthRequest__Output } from '../proto/AuthRequest';
import type { Empty as _proto_Empty, Empty__Output as _proto_Empty__Output } from '../proto/Empty';
import type { File as _proto_File, File__Output as _proto_File__Output } from '../proto/File';
import type { FileId as _proto_FileId, FileId__Output as _proto_FileId__Output } from '../proto/FileId';
import type { PaymentId as _proto_PaymentId, PaymentId__Output as _proto_PaymentId__Output } from '../proto/PaymentId';
import type { TargetId as _proto_TargetId, TargetId__Output as _proto_TargetId__Output } from '../proto/TargetId';
import type { User as _proto_User, User__Output as _proto_User__Output } from '../proto/User';
import type { UserId as _proto_UserId, UserId__Output as _proto_UserId__Output } from '../proto/UserId';
import type { UserIds as _proto_UserIds, UserIds__Output as _proto_UserIds__Output } from '../proto/UserIds';

export interface UsersServiceClient extends grpc.Client {
  Auth(argument: _proto_AuthRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  Auth(argument: _proto_AuthRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  Auth(argument: _proto_AuthRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  Auth(argument: _proto_AuthRequest, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_AuthRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_AuthRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_AuthRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_AuthRequest, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  
  CreateFile(argument: _proto_File, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  CreateFile(argument: _proto_File, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  CreateFile(argument: _proto_File, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  CreateFile(argument: _proto_File, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  createFile(argument: _proto_File, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  createFile(argument: _proto_File, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  createFile(argument: _proto_File, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  createFile(argument: _proto_File, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  
  DeleteFile(argument: _proto_FileId, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  DeleteFile(argument: _proto_FileId, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  DeleteFile(argument: _proto_FileId, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  DeleteFile(argument: _proto_FileId, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  deleteFile(argument: _proto_FileId, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  deleteFile(argument: _proto_FileId, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  deleteFile(argument: _proto_FileId, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  deleteFile(argument: _proto_FileId, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  
  GetFile(argument: _proto_FileId, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  GetFile(argument: _proto_FileId, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  GetFile(argument: _proto_FileId, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  GetFile(argument: _proto_FileId, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFile(argument: _proto_FileId, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFile(argument: _proto_FileId, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFile(argument: _proto_FileId, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  getFile(argument: _proto_FileId, callback: grpc.requestCallback<_proto_File__Output>): grpc.ClientUnaryCall;
  
  GetFiles(argument: _proto_TargetId, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  GetFiles(argument: _proto_TargetId, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  getFiles(argument: _proto_TargetId, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  getFiles(argument: _proto_TargetId, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_File__Output>;
  
  GetUser(argument: _proto_UserId, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  GetUser(argument: _proto_UserId, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  GetUser(argument: _proto_UserId, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  GetUser(argument: _proto_UserId, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  getUser(argument: _proto_UserId, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  getUser(argument: _proto_UserId, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  getUser(argument: _proto_UserId, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  getUser(argument: _proto_UserId, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  
  GetUsers(argument: _proto_UserIds, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_User__Output>;
  GetUsers(argument: _proto_UserIds, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_User__Output>;
  getUsers(argument: _proto_UserIds, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_User__Output>;
  getUsers(argument: _proto_UserIds, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_User__Output>;
  
  UpdatePaymentId(argument: _proto_PaymentId, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  UpdatePaymentId(argument: _proto_PaymentId, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  UpdatePaymentId(argument: _proto_PaymentId, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  UpdatePaymentId(argument: _proto_PaymentId, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  updatePaymentId(argument: _proto_PaymentId, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  updatePaymentId(argument: _proto_PaymentId, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  updatePaymentId(argument: _proto_PaymentId, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  updatePaymentId(argument: _proto_PaymentId, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  
  UpdateUser(argument: _proto_User, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  UpdateUser(argument: _proto_User, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  UpdateUser(argument: _proto_User, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  UpdateUser(argument: _proto_User, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  updateUser(argument: _proto_User, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  updateUser(argument: _proto_User, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  updateUser(argument: _proto_User, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  updateUser(argument: _proto_User, callback: grpc.requestCallback<_proto_Empty__Output>): grpc.ClientUnaryCall;
  
}

export interface UsersServiceHandlers extends grpc.UntypedServiceImplementation {
  Auth: grpc.handleUnaryCall<_proto_AuthRequest__Output, _proto_User>;
  
  CreateFile: grpc.handleUnaryCall<_proto_File__Output, _proto_File>;
  
  DeleteFile: grpc.handleUnaryCall<_proto_FileId__Output, _proto_File>;
  
  GetFile: grpc.handleUnaryCall<_proto_FileId__Output, _proto_File>;
  
  GetFiles: grpc.handleServerStreamingCall<_proto_TargetId__Output, _proto_File>;
  
  GetUser: grpc.handleUnaryCall<_proto_UserId__Output, _proto_User>;
  
  GetUsers: grpc.handleServerStreamingCall<_proto_UserIds__Output, _proto_User>;
  
  UpdatePaymentId: grpc.handleUnaryCall<_proto_PaymentId__Output, _proto_Empty>;
  
  UpdateUser: grpc.handleUnaryCall<_proto_User__Output, _proto_Empty>;
  
}

export interface UsersServiceDefinition extends grpc.ServiceDefinition {
  Auth: MethodDefinition<_proto_AuthRequest, _proto_User, _proto_AuthRequest__Output, _proto_User__Output>
  CreateFile: MethodDefinition<_proto_File, _proto_File, _proto_File__Output, _proto_File__Output>
  DeleteFile: MethodDefinition<_proto_FileId, _proto_File, _proto_FileId__Output, _proto_File__Output>
  GetFile: MethodDefinition<_proto_FileId, _proto_File, _proto_FileId__Output, _proto_File__Output>
  GetFiles: MethodDefinition<_proto_TargetId, _proto_File, _proto_TargetId__Output, _proto_File__Output>
  GetUser: MethodDefinition<_proto_UserId, _proto_User, _proto_UserId__Output, _proto_User__Output>
  GetUsers: MethodDefinition<_proto_UserIds, _proto_User, _proto_UserIds__Output, _proto_User__Output>
  UpdatePaymentId: MethodDefinition<_proto_PaymentId, _proto_Empty, _proto_PaymentId__Output, _proto_Empty__Output>
  UpdateUser: MethodDefinition<_proto_User, _proto_Empty, _proto_User__Output, _proto_Empty__Output>
}
