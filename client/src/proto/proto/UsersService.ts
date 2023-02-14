// Original file: main.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { AuthRequest as _proto_AuthRequest, AuthRequest__Output as _proto_AuthRequest__Output } from '../proto/AuthRequest';
import type { User as _proto_User, User__Output as _proto_User__Output } from '../proto/User';

export interface UsersServiceClient extends grpc.Client {
  Auth(argument: _proto_AuthRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  Auth(argument: _proto_AuthRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  Auth(argument: _proto_AuthRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  Auth(argument: _proto_AuthRequest, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_AuthRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_AuthRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_AuthRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_AuthRequest, callback: grpc.requestCallback<_proto_User__Output>): grpc.ClientUnaryCall;
  
}

export interface UsersServiceHandlers extends grpc.UntypedServiceImplementation {
  Auth: grpc.handleUnaryCall<_proto_AuthRequest__Output, _proto_User>;
  
}

export interface UsersServiceDefinition extends grpc.ServiceDefinition {
  Auth: MethodDefinition<_proto_AuthRequest, _proto_User, _proto_AuthRequest__Output, _proto_User__Output>
}
