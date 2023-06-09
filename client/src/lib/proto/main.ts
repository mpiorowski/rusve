import type * as grpc from '@grpc/grpc-js';
import type { EnumTypeDefinition, MessageTypeDefinition } from '@grpc/proto-loader';

import type { NotesServiceClient as _proto_NotesServiceClient, NotesServiceDefinition as _proto_NotesServiceDefinition } from './proto/NotesService';
import type { UsersServiceClient as _proto_UsersServiceClient, UsersServiceDefinition as _proto_UsersServiceDefinition } from './proto/UsersService';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  proto: {
    AuthRequest: MessageTypeDefinition
    Empty: MessageTypeDefinition
    File: MessageTypeDefinition
    FileId: MessageTypeDefinition
    FileType: EnumTypeDefinition
    Note: MessageTypeDefinition
    NoteId: MessageTypeDefinition
    NotesService: SubtypeConstructor<typeof grpc.Client, _proto_NotesServiceClient> & { service: _proto_NotesServiceDefinition }
    PaymentId: MessageTypeDefinition
    TargetId: MessageTypeDefinition
    User: MessageTypeDefinition
    UserId: MessageTypeDefinition
    UserIds: MessageTypeDefinition
    UserRole: EnumTypeDefinition
    UsersService: SubtypeConstructor<typeof grpc.Client, _proto_UsersServiceClient> & { service: _proto_UsersServiceDefinition }
  }
}

