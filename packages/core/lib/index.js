import path from 'node:path';
import { arch } from 'node:process';
import { fileURLToPath } from 'node:url';
import { EventEmitter } from 'node:events';
export * from './types.js';
export * from './util.js';
/**
 * Global node addon instance
 */
const addon = loadAddon();
/**
 * Load node addon depending on system architecture.
 */
function loadAddon() {
    const nodeModule = { exports: {} };
    let name;
    switch (arch) {
        case 'arm64': {
            name = '../addon-aarch64.node';
            break;
        }
        case 'x64': {
            name = '../addon-x64.node';
            break;
        }
        default: throw new Error(`Unsupported arch: ${arch}`);
    }
    process.dlopen(nodeModule, path.resolve(path.dirname(fileURLToPath(new URL(import.meta.url))), name));
    return nodeModule.exports;
}
/**
 * Unique symbol for accessing internal id
 */
const idSym = Symbol('id');
export class Overlay {
    event = new EventEmitter();
    [idSym];
    constructor(id) {
        this[idSym] = id;
        void (async () => {
            // wait until next tick so no events are lost
            await new Promise((resolve) => {
                process.nextTick(resolve);
            });
            try {
                for (;;) {
                    const hasNext = await addon.overlayCallNextEvent(id, this.event, (name, ...args) => this.event.emit(name, ...args));
                    if (!hasNext)
                        break;
                }
            }
            catch (err) {
                if (this.event.listenerCount('error') != 0) {
                    this.event.emit('error', err);
                }
                else {
                    throw err;
                }
            }
            finally {
                this.destroy();
            }
        })();
    }
    /**
     * Update overlay position relative to window
     * @param id target window id
     * @param x x position
     * @param y y position
     */
    async setPosition(id, x, y) {
        await addon.overlaySetPosition(this[idSym], id, x, y);
    }
    /**
     * Update overlay anchor
     * @param id target window id
     * @param x x anchor
     * @param y y anchor
     */
    async setAnchor(id, x, y) {
        await addon.overlaySetAnchor(this[idSym], id, x, y);
    }
    /**
     * Update overlay margin
     * @param id target window id
     * @param top top margin
     * @param right right margin
     * @param bottom bottom margin
     * @param left left margin
     */
    async setMargin(id, top, right, bottom, left) {
        await addon.overlaySetMargin(this[idSym], id, top, right, bottom, left);
    }
    /**
     * Listen to window input without blocking
     * @param id target window id
     * @param cursor listen cursor input or not
     * @param keyboard listen keyboard input or not
     */
    async listenInput(id, cursor, keyboard) {
        await addon.overlayListenInput(this[idSym], id, cursor, keyboard);
    }
    /**
     * Block window input and listen them
     * @param id target window id
     * @param block set true to block input, false to release
     */
    async blockInput(id, block) {
        await addon.overlayBlockInput(this[idSym], id, block);
    }
    /**
     * Set cursor while in input blocking mode
     * @param id target window id
     * @param cursor cursor to set. Do not supply this value to hide cursor.
     */
    async setBlockingCursor(id, cursor) {
        await addon.overlaySetBlockingCursor(this[idSym], id, cursor);
    }
    /**
     * Update overlay surface.
     * @param id target window id
     * @param update shared handle update
     */
    async updateHandle(id, update) {
        await addon.overlayUpdateHandle(this[idSym], id, update);
    }
    /**
     * Destroy overlay
     */
    destroy() {
        addon.overlayDestroy(this[idSym]);
        this.event.emit('disconnected');
    }
    /**
     * Attach overlay to target process
     *
     * Name must be unique or it will fail if there is a connection with same name
     * @param dllDir path to dlls
     * @param pid target process pid
     * @param timeout Timeout for injection, in milliseconds. Will wait indefinitely if not provided.
     * @returns new {@link Overlay} object
     */
    static async attach(dllDir, pid, timeout) {
        return new Overlay(await addon.attach(dllDir, pid, timeout));
    }
}
/**
 * Represent a surface for overlay.
 */
export class OverlaySurface {
    [idSym];
    constructor(id) {
        this[idSym] = id;
    }
    /**
     * Update surface using bitmap buffer. The size of overlay is `width x (data.byteLength / 4 / width)`
     * @param width width of the bitmap
     * @param data bgra formatted bitmap
     */
    updateBitmap(width, data) {
        return addon.surfaceUpdateBitmap(this[idSym], width, data);
    }
    /**
     * Update surface using D3D11 shared texture.
     * @param width width of the surface
     * @param height height of the surface
     * @param handle NT Handle of shared D3D11 Texture
     * @param rect Area to update
     */
    updateShtex(width, height, handle, rect) {
        return addon.surfaceUpdateShtex(this[idSym], width, height, handle, rect);
    }
    /**
     * Clear the surface.
     */
    clear() {
        addon.surfaceClear(this[idSym]);
    }
    /**
     * Destroy the surface.
     */
    destroy() {
        addon.surfaceDestroy(this[idSym]);
    }
    /**
     * Create a new overlay surface.
     * @param luid The GPU LUID for surface textures.
     */
    static create(luid) {
        return new OverlaySurface(addon.surfaceCreate(luid));
    }
}
/**
 * Default dll directory path
 */
export function defaultDllDir() {
    return path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../');
}
