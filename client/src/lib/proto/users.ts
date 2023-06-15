import type { EnumTypeDefinition, MessageTypeDefinition } from '@grpc/proto-loader';

export interface ProtoGrpcType {
  proto: {
    File: MessageTypeDefinition
    FileType: EnumTypeDefinition
    User: MessageTypeDefinition
    UserRole: EnumTypeDefinition
  }
}

