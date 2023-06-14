// Original file: ../proto/main.proto


export interface PaymentId {
  'userId'?: (Buffer | Uint8Array | string);
  'paymentId'?: (string);
}

export interface PaymentId__Output {
  'userId': (Buffer);
  'paymentId': (string);
}
