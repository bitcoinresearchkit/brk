interface Options {
    /**
     * Width in pixels to be applied to node before rendering.
     */
    width?: number;
    /**
     * Height in pixels to be applied to node before rendering.
     */
    height?: number;
    /**
     * A number between `0` and `1` indicating image quality (e.g. 0.92 => 92%) of the JPEG image.
     */
    quality?: number;
    /**
     * A string indicating the image format. The default type is image/png; that type is also used if the given type isn't supported.
     */
    type?: string;
    /**
     * The pixel ratio of captured image.
     *
     * DPI = 96 * scale
     *
     * default: 1
     */
    scale?: number;
    /**
     * A string value for the background color, any valid CSS color value.
     */
    backgroundColor?: string | null;
    /**
     * An object whose properties to be copied to node's style before rendering.
     */
    style?: Partial<CSSStyleDeclaration> | null;
    /**
     * A function taking DOM node as argument. Should return `true` if passed
     * node should be included in the output. Excluding node means excluding
     * it's children as well.
     */
    filter?: ((el: Node) => boolean) | null;
    /**
     * Maximum canvas size (pixels).
     *
     * https://developer.mozilla.org/en-US/docs/Web/HTML/Element/canvas#maximum_canvas_size
     */
    maximumCanvasSize?: number;
    /**
     * Load media timeout and fetch remote asset timeout (millisecond).
     *
     * default: 30000
     */
    timeout?: number;
    /**
     * Embed assets progress.
     */
    progress?: ((current: number, total: number) => void) | null;
    /**
     * Enable debug mode to view the execution time log.
     */
    debug?: boolean;
    /**
     * Custom implementation to get image data for a custom URL.
     * This can be helpful for Capacitor or Cordova when using
     * native fetch to bypass CORS issues.
     *
     * If returns a string, will completely bypass any `Options.fetch`
     * settings with your custom implementation.
     *
     * If returns false, will fall back to normal fetch implementation
     *
     * @param url
     * @returns A data URL for the image
     */
    fetchFn?: ((url: string) => Promise<string | false>) | null;
    /**
     * The options of fetch resources.
     */
    fetch?: {
        /**
         * The second parameter of `window.fetch` RequestInit
         *
         * default: {
         *   cache: 'force-cache',
         * }
         */
        requestInit?: RequestInit;
        /**
         * Set to `true` to append the current time as a query string to URL
         * requests to enable cache busting.
         *
         * default: false
         */
        bypassingCache?: boolean | RegExp;
        /**
         * A data URL for a placeholder image that will be used when fetching
         * an image fails. Defaults to an empty string and will render empty
         * areas for failed images.
         *
         * default: data:image/png;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7
         */
        placeholderImage?: string | ((cloned: HTMLImageElement | SVGImageElement) => string | Promise<string>);
    };
    /**
     * The options of fonts download and embed.
     */
    font?: false | {
        /**
         * Font minify
         */
        minify?: (font: ArrayBuffer, subset: string) => ArrayBuffer;
        /**
         * The preferred font format. If specified all other font formats are ignored.
         */
        preferredFormat?: 'woff' | 'woff2' | 'truetype' | 'opentype' | 'embedded-opentype' | 'svg' | string;
        /**
         * A CSS string to specify for font embeds. If specified only this CSS will
         * be present in the resulting image.
         */
        cssText?: string;
    };
    /**
     * All enabled features
     *
     * default: true
     */
    features?: boolean | {
        /**
         * Copy scrollbar css styles
         *
         * default: true
         */
        copyScrollbar?: boolean;
        /**
         * Remove abnormal attributes to cloned node (for normalize XML)
         *
         * default: true
         */
        removeAbnormalAttributes?: boolean;
        /**
         * Remove control characters (for normalize XML)
         *
         * default: true
         */
        removeControlCharacter?: boolean;
        /**
         * Fix svg+xml image decode (for Safari、Firefox)
         *
         * default: true
         */
        fixSvgXmlDecode?: boolean;
        /**
         * Render scrolled children with scrolled content
         *
         * default: false
         */
        restoreScrollPosition?: boolean;
    };
    /**
     * Canvas `drawImage` interval
     * is used to fix errors in decoding images in Safari、Firefox
     *
     * default: 100
     */
    drawImageInterval?: number;
    /**
     * Web Worker script url
     */
    workerUrl?: string | null;
    /**
     * Web Worker number
     */
    workerNumber?: number;
    /**
     * Triggered after each node is cloned
     */
    onCloneEachNode?: ((cloned: Node) => void | Promise<void>) | null;
    /**
     * Triggered after a node is cloned
     */
    onCloneNode?: ((cloned: Node) => void | Promise<void>) | null;
    /**
     * Triggered after a node is embed
     */
    onEmbedNode?: ((cloned: Node) => void | Promise<void>) | null;
    /**
     * Triggered after a ForeignObjectSvg is created
     */
    onCreateForeignObjectSvg?: ((svg: SVGSVGElement) => void | Promise<void>) | null;
    /**
     * An array of style property names.
     * Can be used to manually specify which style properties are
     * included when cloning nodes.
     * This can be useful for performance-critical scenarios.
     */
    includeStyleProperties?: string[] | null;
}

interface Request {
    type: 'image' | 'text';
    resolve?: (response: string) => void;
    reject?: (error: Error) => void;
    response: Promise<string>;
}
interface InternalContext<T extends Node> {
    /**
     * FLAG
     */
    __CONTEXT__: true;
    /**
     * Logger
     */
    log: {
        time: (label: string) => void;
        timeEnd: (label: string) => void;
        warn: (...args: any[]) => void;
    };
    /**
     * Node
     */
    node: T;
    /**
     * Owner document
     */
    ownerDocument?: Document;
    /**
     * Owner window
     */
    ownerWindow?: Window;
    /**
     * DPI
     *
     * scale === 1 ? null : 96 * scale
     */
    dpi: number | null;
    /**
     * The `style` element under the root `svg` element
     */
    svgStyleElement?: HTMLStyleElement;
    /**
     * The `defs` element under the root `svg` element
     */
    svgDefsElement?: SVGDefsElement;
    /**
     * The `svgStyleElement` class styles
     *
     * Map<cssText, class[]>
     */
    svgStyles: Map<string, string[]>;
    /**
     * The map of default `getComputedStyle` for all tagnames
     */
    defaultComputedStyles: Map<string, Map<string, any>>;
    /**
     * The IFrame sandbox used to get the `defaultComputedStyles`
     */
    sandbox?: HTMLIFrameElement;
    /**
     * Web Workers
     */
    workers: Worker[];
    /**
     * The map of `font-family` values for all cloend elements
     */
    fontFamilies: Map<string, Set<string>>;
    /**
     * Map<CssUrl, DataUrl>
     */
    fontCssTexts: Map<string, string>;
    /**
     * `headers.accept` to use when `window.fetch` fetches images
     */
    acceptOfImage: string;
    /**
     * All requests for `fetch`
     */
    requests: Map<string, Request>;
    /**
     * Canvas multiple draw image fix svg+xml image decoding in Safari and Firefox
     */
    drawImageCount: number;
    /**
     * Wait for all tasks embedded in
     */
    tasks: Promise<void>[];
    /**
     * Automatically destroy context
     */
    autoDestruct: boolean;
    /**
     * Is enable
     *
     * @param key
     */
    isEnable: (key: string) => boolean;
    /**
     * [cloning phase] To get the node style set by the user
     */
    currentNodeStyle?: Map<string, [string, string]>;
    currentParentNodeStyle?: Map<string, [string, string]>;
    /**
     * [cloning phase] shadowDOM root list
     */
    shadowRoots: ShadowRoot[];
}
type Context<T extends Node = Node> = InternalContext<T> & Required<Options>;

declare function domToBlob<T extends Node>(node: T, options?: Options): Promise<Blob>;
declare function domToBlob<T extends Node>(context: Context<T>): Promise<Blob>;

declare function domToCanvas<T extends Node>(node: T, options?: Options): Promise<HTMLCanvasElement>;
declare function domToCanvas<T extends Node>(context: Context<T>): Promise<HTMLCanvasElement>;

declare function domToDataUrl<T extends Node>(node: T, options?: Options): Promise<string>;
declare function domToDataUrl<T extends Node>(context: Context<T>): Promise<string>;

declare function domToForeignObjectSvg<T extends Node>(node: T, options?: Options): Promise<SVGElement>;
declare function domToForeignObjectSvg<T extends Node>(context: Context<T>): Promise<SVGElement>;

declare function domToImage<T extends Node>(node: T, options?: Options): Promise<HTMLImageElement>;
declare function domToImage<T extends Node>(context: Context<T>): Promise<HTMLImageElement>;

declare function domToJpeg<T extends Node>(node: T, options?: Options): Promise<string>;
declare function domToJpeg<T extends Node>(context: Context<T>): Promise<string>;

declare function domToPixel<T extends Node>(node: T, options?: Options): Promise<Uint8ClampedArray>;
declare function domToPixel<T extends Node>(context: Context<T>): Promise<Uint8ClampedArray>;

declare function domToPng<T extends Node>(node: T, options?: Options): Promise<string>;
declare function domToPng<T extends Node>(context: Context<T>): Promise<string>;

declare function domToSvg<T extends Node>(node: T, options?: Options): Promise<string>;
declare function domToSvg<T extends Node>(context: Context<T>): Promise<string>;

declare function domToWebp<T extends Node>(node: T, options?: Options): Promise<string>;
declare function domToWebp<T extends Node>(context: Context<T>): Promise<string>;

declare function createContext<T extends Node>(node: T, options?: Options & {
    autoDestruct?: boolean;
}): Promise<Context<T>>;

declare function destroyContext(context: Context): void;

type Media = HTMLVideoElement | HTMLImageElement | SVGImageElement;
interface LoadMediaOptions {
    ownerDocument?: Document;
    timeout?: number;
    onError?: (error: Error) => void;
    onWarn?: (...args: any[]) => void;
}
declare function loadMedia<T extends Media>(media: T, options?: LoadMediaOptions): Promise<T>;
declare function loadMedia(media: string, options?: LoadMediaOptions): Promise<HTMLImageElement>;
declare function waitUntilLoad(node: Node, options?: LoadMediaOptions): Promise<void>;

export { type Context, type Options, createContext, destroyContext, domToBlob, domToCanvas, domToDataUrl, domToForeignObjectSvg, domToImage, domToJpeg, domToPixel, domToPng, domToSvg, domToWebp, loadMedia, waitUntilLoad };
