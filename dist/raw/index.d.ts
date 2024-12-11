/* tslint:disable */
/* eslint-disable */
export function hashSync(data: string, salt_or_rounds?: number): string;
export function compareSync(data: string, encrypted: string): boolean;
export function hash(data: string, salt_or_rounds?: number): Promise<any>;
export function compare(data: string, encrypted: string): Promise<any>;
