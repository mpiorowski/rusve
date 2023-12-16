// Original file: main.proto

import type { Long } from '@grpc/proto-loader';

export interface Page {
  'offset'?: (number | string | Long);
  'limit'?: (number | string | Long);
}

export interface Page__Output {
  'offset': (string);
  'limit': (string);
}
