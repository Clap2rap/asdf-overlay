import { EventEmitter } from 'node:events';
import { CursorInput, KeyboardInput } from './input.js';
import { PercentLength, CopyRect, Cursor, type UpdateSharedHandle, type GpuLuid } from './types.js';
export * from './types.js';
export * from './util.js';
/**
 * Unique symbol for accessing internal id
 */
declare const idSym: unique symbol;
export type OverlayEventEmitter = EventEmitter<{
    /**
     * A window has been added.
     */
    added: [id: number, width: number, height: number, luid: GpuLuid];
    /**
     * A window has been resized.
     */
    resized: [id: number, width: number, height: number];
    /**
     * Cursor input from a window.
     */
    cursor_input: [id: number, input: CursorInput];
    /**
     * Keyboard input from a window.
     */
    keyboard_input: [id: number, input: KeyboardInput];
    /**
     * Input blocking to a window is interrupted and turned off.
     */
    input_blocking_ended: [id: number];
    /**
     * Window is destroyed.
     */
    destroyed: [id: number];
    /**
     * An error has occured on ipc connection.
     */
    error: [err: unknown];
    /**
     * Ipc disconnected.
     */
    disconnected: [];
}>;
export declare class Overlay {
    readonly event: OverlayEventEmitter;
    readonly [idSym]: unknown;
    private constructor();
    /**
     * Update overlay position relative to window
     * @param id target window id
     * @param x x position
     * @param y y position
     */
    setPosition(id: number, x: PercentLength, y: PercentLength): Promise<void>;
    /**
     * Update overlay anchor
     * @param id target window id
     * @param x x anchor
     * @param y y anchor
     */
    setAnchor(id: number, x: PercentLength, y: PercentLength): Promise<void>;
    /**
     * Update overlay margin
     * @param id target window id
     * @param top top margin
     * @param right right margin
     * @param bottom bottom margin
     * @param left left margin
     */
    setMargin(id: number, top: PercentLength, right: PercentLength, bottom: PercentLength, left: PercentLength): Promise<void>;
    /**
     * Listen to window input without blocking
     * @param id target window id
     * @param cursor listen cursor input or not
     * @param keyboard listen keyboard input or not
     */
    listenInput(id: number, cursor: boolean, keyboard: boolean): Promise<void>;
    /**
     * Block window input and listen them
     * @param id target window id
     * @param block set true to block input, false to release
     */
    blockInput(id: number, block: boolean): Promise<void>;
    /**
     * Set cursor while in input blocking mode
     * @param id target window id
     * @param cursor cursor to set. Do not supply this value to hide cursor.
     */
    setBlockingCursor(id: number, cursor?: Cursor): Promise<void>;
    /**
     * Update overlay surface.
     * @param id target window id
     * @param update shared handle update
     */
    updateHandle(id: number, update: UpdateSharedHandle): Promise<void>;
    /**
     * Destroy overlay
     */
    destroy(): void;
    /**
     * Attach overlay to target process
     *
     * Name must be unique or it will fail if there is a connection with same name
     * @param dllDir path to dlls
     * @param pid target process pid
     * @param timeout Timeout for injection, in milliseconds. Will wait indefinitely if not provided.
     * @returns new {@link Overlay} object
     */
    static attach(dllDir: string, pid: number, timeout?: number): Promise<Overlay>;
}
/**
 * Represent a surface for overlay.
 */
export declare class OverlaySurface {
    readonly [idSym]: unknown;
    private constructor();
    /**
     * Update surface using bitmap buffer. The size of overlay is `width x (data.byteLength / 4 / width)`
     * @param width width of the bitmap
     * @param data bgra formatted bitmap
     */
    updateBitmap(width: number, data: Buffer): UpdateSharedHandle | null;
    /**
     * Update surface using D3D11 shared texture.
     * @param width width of the surface
     * @param height height of the surface
     * @param handle NT Handle of shared D3D11 Texture
     * @param rect Area to update
     */
    updateShtex(width: number, height: number, handle: Buffer, rect?: CopyRect): UpdateSharedHandle | null;
    /**
     * Clear the surface.
     */
    clear(): void;
    /**
     * Destroy the surface.
     */
    destroy(): void;
    /**
     * Create a new overlay surface.
     * @param luid The GPU LUID for surface textures.
     */
    static create(luid: GpuLuid): OverlaySurface;
}
/**
 * Default dll directory path
 */
export declare function defaultDllDir(): string;
