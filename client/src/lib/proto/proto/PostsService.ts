// Original file: ../proto/main.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { Empty as _proto_Empty, Empty__Output as _proto_Empty__Output } from '../proto/Empty';
import type { Post as _proto_Post, Post__Output as _proto_Post__Output } from '../proto/Post';
import type { PostId as _proto_PostId, PostId__Output as _proto_PostId__Output } from '../proto/PostId';

export interface PostsServiceClient extends grpc.Client {
  CreatePost(argument: _proto_Post, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  CreatePost(argument: _proto_Post, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  CreatePost(argument: _proto_Post, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  CreatePost(argument: _proto_Post, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  createPost(argument: _proto_Post, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  createPost(argument: _proto_Post, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  createPost(argument: _proto_Post, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  createPost(argument: _proto_Post, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  
  DeletePost(argument: _proto_PostId, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  DeletePost(argument: _proto_PostId, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  DeletePost(argument: _proto_PostId, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  DeletePost(argument: _proto_PostId, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  deletePost(argument: _proto_PostId, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  deletePost(argument: _proto_PostId, metadata: grpc.Metadata, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  deletePost(argument: _proto_PostId, options: grpc.CallOptions, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  deletePost(argument: _proto_PostId, callback: grpc.requestCallback<_proto_Post__Output>): grpc.ClientUnaryCall;
  
  GetPosts(argument: _proto_Empty, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Post__Output>;
  GetPosts(argument: _proto_Empty, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Post__Output>;
  getPosts(argument: _proto_Empty, metadata: grpc.Metadata, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Post__Output>;
  getPosts(argument: _proto_Empty, options?: grpc.CallOptions): grpc.ClientReadableStream<_proto_Post__Output>;
  
}

export interface PostsServiceHandlers extends grpc.UntypedServiceImplementation {
  CreatePost: grpc.handleUnaryCall<_proto_Post__Output, _proto_Post>;
  
  DeletePost: grpc.handleUnaryCall<_proto_PostId__Output, _proto_Post>;
  
  GetPosts: grpc.handleServerStreamingCall<_proto_Empty__Output, _proto_Post>;
  
}

export interface PostsServiceDefinition extends grpc.ServiceDefinition {
  CreatePost: MethodDefinition<_proto_Post, _proto_Post, _proto_Post__Output, _proto_Post__Output>
  DeletePost: MethodDefinition<_proto_PostId, _proto_Post, _proto_PostId__Output, _proto_Post__Output>
  GetPosts: MethodDefinition<_proto_Empty, _proto_Post, _proto_Empty__Output, _proto_Post__Output>
}
