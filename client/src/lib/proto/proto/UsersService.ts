// Original file: ../proto/main.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { AuthRequest as _proto_AuthRequest, AuthRequest__Output as _proto_AuthRequest__Output } from '../proto/AuthRequest';
import type { Empty as _proto_Empty, Empty__Output as _proto_Empty__Output } from '../proto/Empty';
import type { PaymentId as _proto_PaymentId, PaymentId__Output as _proto_PaymentId__Output } from '../proto/PaymentId';
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
  
  CreateUser(argument: _proto_User, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  CreateUser(argument: _proto_User, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  CreateUser(argument: _proto_User, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  CreateUser(argument: _proto_User, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  createUser(argument: _proto_User, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  createUser(argument: _proto_User, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  createUser(argument: _proto_User, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  createUser(argument: _proto_User, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  
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
  
}

export interface UsersServiceHandlers extends grpc.UntypedServiceImplementation {
  Auth: grpc.handleUnaryCall<_proto_AuthRequest__Output, _proto_User>;
  
  CreateUser: grpc.handleUnaryCall<_proto_User__Output, _proto_User>;
  
  GetUser: grpc.handleUnaryCall<_proto_UserId__Output, _proto_User>;
  
  GetUsers: grpc.handleServerStreamingCall<_proto_UserIds__Output, _proto_User>;
  
  UpdatePaymentId: grpc.handleUnaryCall<_proto_PaymentId__Output, _proto_Empty>;
  
}

export interface UsersServiceDefinition extends grpc.ServiceDefinition {
  Auth: MethodDefinition<_proto_AuthRequest, _proto_User, _proto_AuthRequest__Output, _proto_User__Output>
  CreateUser: MethodDefinition<_proto_User, _proto_User, _proto_User__Output, _proto_User__Output>
  GetUser: MethodDefinition<_proto_UserId, _proto_User, _proto_UserId__Output, _proto_User__Output>
  GetUsers: MethodDefinition<_proto_UserIds, _proto_User, _proto_UserIds__Output, _proto_User__Output>
  UpdatePaymentId: MethodDefinition<_proto_PaymentId, _proto_Empty, _proto_PaymentId__Output, _proto_Empty__Output>
}
