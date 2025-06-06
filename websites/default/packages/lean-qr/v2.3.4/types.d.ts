declare module "lean-qr" {
  interface ImageDataLike {
    readonly data: Uint8ClampedArray;
  }

  interface Context2DLike<DataT extends ImageDataLike> {
    createImageData(width: number, height: number): DataT;
    putImageData(data: DataT, x: number, y: number): void;
  }

  interface CanvasLike<DataT extends ImageDataLike> {
    width: number;
    height: number;
    getContext(type: "2d"): Context2DLike<DataT> | null;
  }

  export type RGBA = readonly [number, number, number, number?];

  export interface Bitmap1D {
    push(value: number, bits: number): void;
  }

  export interface StringOptions {
    on?: string;
    off?: string;
    lf?: string;
    padX?: number;
    padY?: number;
  }

  export interface ImageDataOptions {
    on?: RGBA;
    off?: RGBA;
    padX?: number;
    padY?: number;
  }

  export interface Bitmap2D {
    readonly size: number;

    get(x: number, y: number): boolean;

    toString(options?: Readonly<StringOptions>): string;

    toImageData<DataT extends ImageDataLike>(
      context: Context2DLike<DataT>,
      options?: Readonly<ImageDataOptions>,
    ): DataT;

    toDataURL(
      options?: Readonly<
        ImageDataOptions & {
          type?: `image/${string}`;
          scale?: number;
        }
      >,
    ): string;

    toCanvas(
      canvas: CanvasLike<ImageDataLike>,
      options?: Readonly<ImageDataOptions>,
    ): void;
  }

  export type Mask = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7;
  export type Mode = (data: Bitmap1D, version: number) => void;
  export interface ModeFactory {
    (value: string): Mode;
    test(string: string): boolean;
    est(value: string, version: number): number;
    eci?: number;
  }

  interface ModeAutoOptions {
    modes?: ReadonlyArray<ModeFactory>;
  }

  export const mode: Readonly<{
    auto(value: string, options?: Readonly<ModeAutoOptions>): Mode;
    multi(...modes: ReadonlyArray<Mode>): Mode;
    eci(id: number): Mode;
    numeric: ModeFactory;
    alphaNumeric: ModeFactory;
    bytes(data: Uint8Array | ReadonlyArray<number>): Mode;
    ascii: ModeFactory;
    iso8859_1: ModeFactory;
    shift_jis: ModeFactory;
    utf8: ModeFactory;
  }>;

  type Correction = number & { readonly _: unique symbol };
  export const correction: Readonly<{
    min: Correction;
    L: Correction;
    M: Correction;
    Q: Correction;
    H: Correction;
    max: Correction;
  }>;

  export interface GenerateOptions extends ModeAutoOptions {
    minCorrectionLevel?: Correction;
    maxCorrectionLevel?: Correction;
    minVersion?: number;
    maxVersion?: number;
    mask?: null | Mask;
    trailer?: number;
  }

  export type GenerateFn = (
    data: Mode | string,
    options?: Readonly<GenerateOptions>,
  ) => Bitmap2D;
  interface Generate extends GenerateFn {
    with(...modes: ReadonlyArray<ModeFactory>): GenerateFn;
  }
  export const generate: Generate;
}

declare module "lean-qr/extras/svg" {
  import type { Bitmap2D } from "lean-qr";

  export interface SVGOptions {
    on?: string;
    off?: string;
    padX?: number;
    padY?: number;
    width?: number | null;
    height?: number | null;
    scale?: number;
  }

  export const toSvgPath: (code: Bitmap2D) => string;

  export const toSvg: (
    code: Bitmap2D,
    target: Document | SVGElement,
    options?: Readonly<SVGOptions>,
  ) => SVGElement;

  export const toSvgSource: (
    code: Bitmap2D,
    options?: Readonly<SVGOptions & { xmlDeclaration?: boolean }>,
  ) => string;

  export type toSvgDataURLFn = (
    code: Bitmap2D,
    options?: Readonly<SVGOptions>,
  ) => string;
  export const toSvgDataURL: toSvgDataURLFn;
}

declare module "lean-qr/extras/node_export" {
  import type { RGBA, Bitmap2D } from "lean-qr";

  export interface PNGOptions {
    on?: RGBA;
    off?: RGBA;
    padX?: number;
    padY?: number;
    scale?: number;
  }

  export const toPngBuffer: (
    code: Bitmap2D,
    options?: Readonly<PNGOptions>,
  ) => Uint8Array;

  export const toPngDataURL: (
    code: Bitmap2D,
    options?: Readonly<PNGOptions>,
  ) => string;
}

declare module "lean-qr/extras/react" {
  import type { ImageDataOptions, GenerateOptions, GenerateFn } from "lean-qr";
  import type { SVGOptions, toSvgDataURLFn } from "lean-qr/extras/svg";

  export interface AsyncFramework<T> {
    createElement: (
      type: "canvas",
      props: {
        ref: any;
        style: { imageRendering: "pixelated" };
        className: string;
      },
    ) => T;
    useRef<T>(initialValue: T | null): { readonly current: T | null };
    useEffect(fn: () => void | (() => void), deps: unknown[]): void;
  }

  interface QRComponentProps {
    content: string;
    className?: string;
  }

  export interface AsyncQRComponentProps
    extends ImageDataOptions,
      GenerateOptions,
      QRComponentProps {}

  export type AsyncQRComponent<T> = (
    props: Readonly<AsyncQRComponentProps>,
  ) => T;

  export const makeAsyncComponent: <T>(
    framework: Readonly<AsyncFramework<T>>,
    generate: GenerateFn,
    defaultProps?: Readonly<Partial<AsyncQRComponentProps>>,
  ) => AsyncQRComponent<T>;

  export interface SyncFramework<T> {
    createElement: (
      type: "img",
      props: {
        src: string;
        style: { imageRendering: "pixelated" };
        className: string;
      },
    ) => T;
    useMemo<T>(fn: () => T, deps: unknown[]): T;
  }

  export interface SyncQRComponentProps
    extends SVGOptions,
      GenerateOptions,
      QRComponentProps {}

  export type SyncQRComponent<T> = (props: Readonly<SyncQRComponentProps>) => T;

  export const makeSyncComponent: <T>(
    framework: Readonly<SyncFramework<T>>,
    generate: GenerateFn,
    toSvgDataURL: toSvgDataURLFn,
    defaultProps?: Readonly<Partial<SyncQRComponentProps>>,
  ) => SyncQRComponent<T>;
}

declare module "lean-qr/extras/errors" {
  export const readError: (error: unknown) => string;
}
