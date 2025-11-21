/**
 * A length which can be expressed in absolute or relative percent.
 */
export type PercentLength = {
    ty: 'percent' | 'length';
    value: number;
};
/**
 * Locally unique identifier for a GPU.
 */
export type GpuLuid = {
    /**
     * Low part of the LUID.
     */
    low: number;
    /**
     * High part of the LUID.
     */
    high: number;
};
/**
 * Describe a update of overlay surface texture handle.
 */
export type UpdateSharedHandle = {
    /**
     * KMT handle to surface texture.
     *
     * Not supplying this value will clear the existing texture.
     */
    handle?: number;
};
/**
 * Describe a rectangle when copying from source to destination.
 */
export type CopyRect = {
    /**
     * Destination X position.
     */
    dstX: number;
    /**
     * Destination Y position.
     */
    dstY: number;
    /**
     * Source rectangle.
     */
    src: Rect;
};
/**
 * Describe a Reactangle.
 */
export type Rect = {
    /**
     * X position.
     */
    x: number;
    /**
     * Y position.
     */
    y: number;
    /**
     * Width of the Rectangle.
     */
    width: number;
    /**
     * Height of the Rectangle.
     */
    height: number;
};
/**
 * A keyboard key.
 */
export type Key = {
    /**
     * Windows virtual key code.
     */
    code: number;
    /**
     * Extended flag.
     *
     * True for right key variant (e.g. Right shift), or Numpad variant (e.g. NumPad 1)
     */
    extended: boolean;
};
/**
 * Describe a cursor type.
 */
export declare enum Cursor {
    Default = 0,
    Help = 1,
    Pointer = 2,
    Progress = 3,
    Wait = 4,
    Cell = 5,
    Crosshair = 6,
    Text = 7,
    VerticalText = 8,
    Alias = 9,
    Copy = 10,
    Move = 11,
    NotAllowed = 12,
    Grab = 13,
    Grabbing = 14,
    ColResize = 15,
    RowResize = 16,
    EastWestResize = 17,
    NorthSouthResize = 18,
    NorthEastSouthWestResize = 19,
    NorthWestSouthEastResize = 20,
    ZoomIn = 21,
    ZoomOut = 22,
    UpArrow = 23,
    Pin = 24,
    Person = 25,
    Pen = 26,
    Cd = 27,
    PanMiddle = 28,
    PanMiddleHorizontal = 29,
    PanMiddleVertical = 30,
    PanEast = 31,
    PanNorth = 32,
    PanNorthEast = 33,
    PanNorthWest = 34,
    PanSouth = 35,
    PanSouthEast = 36,
    PanSouthWest = 37,
    PanWest = 38
}
