/* tslint:disable */

/* eslint-disable */

declare function hashSync(data: string, salt_or_rounds?: number): string;

declare function compareSync(data: string, encrypted: string): boolean;

declare function hash(data: string, salt_or_rounds?: number): Promise<any>;

declare function compare(data: string, encrypted: string): Promise<any>;

declare namespace _default {
    export {
        hashSync,
        compareSync,
        hash,
        compare
    }
}

export default _default;