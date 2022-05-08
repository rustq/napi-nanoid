/* eslint-disable */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export function nanoid(): string
export function customSize(size?: number | undefined | null): string
export function customAlphabet(size?: number | undefined | null, alphabet?: string | undefined | null): string
