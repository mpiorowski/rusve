// Original file: main.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { AuthResponse as _proto_AuthResponse, AuthResponse__Output as _proto_AuthResponse__Output } from '../proto/AuthResponse';
import type { CreateUserRequest as _proto_CreateUserRequest, CreateUserRequest__Output as _proto_CreateUserRequest__Output } from '../proto/CreateUserRequest';
import type { Empty as _proto_Empty, Empty__Output as _proto_Empty__Output } from '../proto/Empty';
import type { Id as _proto_Id, Id__Output as _proto_Id__Output } from '../proto/Id';
import type { Profile as _proto_Profile, Profile__Output as _proto_Profile__Output } from '../proto/Profile';
import type { StripeUrlResponse as _proto_StripeUrlResponse, StripeUrlResponse__Output as _proto_StripeUrlResponse__Output } from '../proto/StripeUrlResponse';

export interface UsersServiceClient extends grpc.Client {
  Auth(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_AuthResponse__Output>): grpc.ClientUnaryCall;
  Auth(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_AuthResponse__Output>): grpc.ClientUnaryCall;
  Auth(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_AuthResponse__Output>): grpc.ClientUnaryCall;
  Auth(argument: _proto_Empty, callback: grpc.requestCallback<_proto_AuthResponse__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_AuthResponse__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_AuthResponse__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_AuthResponse__Output>): grpc.ClientUnaryCall;
  auth(argument: _proto_Empty, callback: grpc.requestCallback<_proto_AuthResponse__Output>): grpc.ClientUnaryCall;
  
  CreateProfile(argument: _proto_Profile, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  CreateProfile(argument: _proto_Profile, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  CreateProfile(argument: _proto_Profile, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  CreateProfile(argument: _proto_Profile, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  createProfile(argument: _proto_Profile, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  createProfile(argument: _proto_Profile, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  createProfile(argument: _proto_Profile, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  createProfile(argument: _proto_Profile, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  
  CreateStripeCheckout(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  CreateStripeCheckout(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  CreateStripeCheckout(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  CreateStripeCheckout(argument: _proto_Empty, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  createStripeCheckout(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  createStripeCheckout(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  createStripeCheckout(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  createStripeCheckout(argument: _proto_Empty, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  
  CreateStripePortal(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  CreateStripePortal(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  CreateStripePortal(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  CreateStripePortal(argument: _proto_Empty, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  createStripePortal(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  createStripePortal(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  createStripePortal(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  createStripePortal(argument: _proto_Empty, callback: grpc.requestCallback<_proto_StripeUrlResponse__Output>): grpc.ClientUnaryCall;
  
  CreateUser(argument: _proto_CreateUserRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Id__Output>): grpc.ClientUnaryCall;
  CreateUser(argument: _proto_CreateUserRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Id__Output>): grpc.ClientUnaryCall;
  CreateUser(argument: _proto_CreateUserRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Id__Output>): grpc.ClientUnaryCall;
  CreateUser(argument: _proto_CreateUserRequest, callback: grpc.requestCallback<_proto_Id__Output>): grpc.ClientUnaryCall;
  createUser(argument: _proto_CreateUserRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Id__Output>): grpc.ClientUnaryCall;
  createUser(argument: _proto_CreateUserRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Id__Output>): grpc.ClientUnaryCall;
  createUser(argument: _proto_CreateUserRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Id__Output>): grpc.ClientUnaryCall;
  createUser(argument: _proto_CreateUserRequest, callback: grpc.requestCallback<_proto_Id__Output>): grpc.ClientUnaryCall;
  
  GetProfileByUserId(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  GetProfileByUserId(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  GetProfileByUserId(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  GetProfileByUserId(argument: _proto_Empty, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  getProfileByUserId(argument: _proto_Empty, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  getProfileByUserId(argument: _proto_Empty, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  getProfileByUserId(argument: _proto_Empty, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  getProfileByUserId(argument: _proto_Empty, callback: grpc.requestCallback<_proto_Profile__Output>): grpc.ClientUnaryCall;
  
}

export interface UsersServiceHandlers extends grpc.UntypedServiceImplementation {
  Auth: grpc.handleUnaryCall<_proto_Empty__Output, _proto_AuthResponse>;
  
  CreateProfile: grpc.handleUnaryCall<_proto_Profile__Output, _proto_Profile>;
  
  CreateStripeCheckout: grpc.handleUnaryCall<_proto_Empty__Output, _proto_StripeUrlResponse>;
  
  CreateStripePortal: grpc.handleUnaryCall<_proto_Empty__Output, _proto_StripeUrlResponse>;
  
  CreateUser: grpc.handleUnaryCall<_proto_CreateUserRequest__Output, _proto_Id>;
  
  GetProfileByUserId: grpc.handleUnaryCall<_proto_Empty__Output, _proto_Profile>;
  
}

export interface UsersServiceDefinition extends grpc.ServiceDefinition {
  Auth: MethodDefinition<_proto_Empty, _proto_AuthResponse, _proto_Empty__Output, _proto_AuthResponse__Output>
  CreateProfile: MethodDefinition<_proto_Profile, _proto_Profile, _proto_Profile__Output, _proto_Profile__Output>
  CreateStripeCheckout: MethodDefinition<_proto_Empty, _proto_StripeUrlResponse, _proto_Empty__Output, _proto_StripeUrlResponse__Output>
  CreateStripePortal: MethodDefinition<_proto_Empty, _proto_StripeUrlResponse, _proto_Empty__Output, _proto_StripeUrlResponse__Output>
  CreateUser: MethodDefinition<_proto_CreateUserRequest, _proto_Id, _proto_CreateUserRequest__Output, _proto_Id__Output>
  GetProfileByUserId: MethodDefinition<_proto_Empty, _proto_Profile, _proto_Empty__Output, _proto_Profile__Output>
}
