// Original file: main.proto

import type { Long } from '@grpc/proto-loader';

export interface Count {
  'count'?: (number | string | Long);
}

export interface Count__Output {
  'count': (string);
}
