/**
 * Utility function to create `PercentLength` using percent relative value.
 */
export function percent(value) {
    return {
        ty: 'percent',
        value,
    };
}
/**
 * Utilty function to create `PercentLength` using absolute length value.
 */
export function length(value) {
    return {
        ty: 'length',
        value,
    };
}
/**
 * Utility function to create `Key` using key code and optional extended flag.
 */
export function key(code, extended = false) {
    return {
        code,
        extended,
    };
}
