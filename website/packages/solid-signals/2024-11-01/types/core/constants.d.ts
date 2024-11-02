/**
 * See https://dev.to/modderme123/super-charging-fine-grained-reactive-performance-47ph
 * State clean corresponds to a node where all the sources are fully up to date
 * State check corresponds to a node where some sources (including grandparents) may have changed
 * State dirty corresponds to a node where the direct parents of a node has changed
 */
export declare const STATE_CLEAN = 0;
export declare const STATE_CHECK = 1;
export declare const STATE_DIRTY = 2;
export declare const STATE_DISPOSED = 3;
