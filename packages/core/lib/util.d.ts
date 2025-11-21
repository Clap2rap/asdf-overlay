import { Key, PercentLength } from './index.js';
/**
 * Utility function to create `PercentLength` using percent relative value.
 */
export declare function percent(value: number): PercentLength;
/**
 * Utilty function to create `PercentLength` using absolute length value.
 */
export declare function length(value: number): PercentLength;
/**
 * Utility function to create `Key` using key code and optional extended flag.
 */
export declare function key(code: number, extended?: boolean): Key;
