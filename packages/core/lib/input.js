/**
 * IME conversion bit flags.
 *
 * There can be multiple flag set.
 */
export var ImeConversion;
(function (ImeConversion) {
    ImeConversion[ImeConversion["None"] = 0] = "None";
    /**
     * IME converts to native langauge.
     */
    ImeConversion[ImeConversion["Native"] = 1] = "Native";
    /**
     * IME composes in full-width characters.
     */
    ImeConversion[ImeConversion["Fullshape"] = 2] = "Fullshape";
    /**
     * Conversion is disabled.
     */
    ImeConversion[ImeConversion["NoConversion"] = 4] = "NoConversion";
    /**
     * Converting to hanja.
     */
    ImeConversion[ImeConversion["HanjaConvert"] = 8] = "HanjaConvert";
    /**
     * Converting to katakana.
     */
    ImeConversion[ImeConversion["Katakana"] = 16] = "Katakana";
})(ImeConversion || (ImeConversion = {}));
