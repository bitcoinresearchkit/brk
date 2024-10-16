// @ts-nocheck

// src/error.ts
var NotReadyError = class extends Error {};
var NoOwnerError = class extends Error {
  constructor() {
    super("");
  }
};
var ContextNotFoundError = class extends Error {
  constructor() {
    super("");
  }
};

// src/constants.ts
var STATE_CLEAN = 0;
var STATE_CHECK = 1;
var STATE_DIRTY = 2;
var STATE_DISPOSED = 3;

// src/utils.ts
function isUndefined(value) {
  return typeof value === "undefined";
}

// src/owner.ts
var currentOwner = null;
var defaultContext = {};
function getOwner() {
  return currentOwner;
}
function setOwner(owner) {
  const out = currentOwner;
  currentOwner = owner;
  return out;
}
var Owner = class {
  // We flatten the owner tree into a linked list so that we don't need a pointer to .firstChild
  // However, the children are actually added in reverse creation order
  // See comment at the top of the file for an example of the _nextSibling traversal
  o = null;
  k = null;
  n = null;
  a = STATE_CLEAN;
  f = null;
  l = defaultContext;
  g = null;
  constructor(signal = false) {
    if (currentOwner && !signal) currentOwner.append(this);
  }
  append(child) {
    child.o = this;
    child.n = this;
    if (this.k) this.k.n = child;
    child.k = this.k;
    this.k = child;
    if (child.l !== this.l) {
      child.l = { ...this.l, ...child.l };
    }
    if (this.g) {
      child.g = !child.g ? this.g : [...child.g, ...this.g];
    }
  }
  dispose(self = true) {
    if (this.a === STATE_DISPOSED) return;
    let head = self ? this.n || this.o : this,
      current = this.k,
      next = null;
    while (current && current.o === this) {
      current.dispose(true);
      current.t();
      next = current.k;
      current.k = null;
      current = next;
    }
    if (self) this.t();
    if (current) current.n = !self ? this : this.n;
    if (head) head.k = current;
  }
  t() {
    if (this.n) this.n.k = null;
    this.o = null;
    this.n = null;
    this.l = defaultContext;
    this.g = null;
    this.a = STATE_DISPOSED;
    this.emptyDisposal();
  }
  emptyDisposal() {
    if (!this.f) return;
    if (Array.isArray(this.f)) {
      for (let i = 0; i < this.f.length; i++) {
        const callable = this.f[i];
        callable.call(callable);
      }
    } else {
      this.f.call(this.f);
    }
    this.f = null;
  }
  handleError(error) {
    if (!this.g) throw error;
    let i = 0,
      len = this.g.length;
    for (i = 0; i < len; i++) {
      try {
        this.g[i](error);
        break;
      } catch (e) {
        error = e;
      }
    }
    if (i === len) throw error;
  }
};
function createContext(defaultValue, description) {
  return { id: Symbol(description), defaultValue };
}
function getContext(context, owner = currentOwner) {
  if (!owner) {
    throw new NoOwnerError();
  }
  const value = hasContext(context, owner)
    ? owner.l[context.id]
    : context.defaultValue;
  if (isUndefined(value)) {
    throw new ContextNotFoundError();
  }
  return value;
}
function setContext(context, value, owner = currentOwner) {
  if (!owner) {
    throw new NoOwnerError();
  }
  owner.l = {
    ...owner.l,
    [context.id]: isUndefined(value) ? context.defaultValue : value,
  };
}
function hasContext(context, owner = currentOwner) {
  return !isUndefined(owner?.l[context.id]);
}
function onCleanup(disposable) {
  if (!currentOwner) return;
  const node = currentOwner;
  if (!node.f) {
    node.f = disposable;
  } else if (Array.isArray(node.f)) {
    node.f.push(disposable);
  } else {
    node.f = [node.f, disposable];
  }
}

// src/flags.ts
var ERROR_OFFSET = 0;
var ERROR_BIT = 1 << ERROR_OFFSET;
var LOADING_OFFSET = 1;
var LOADING_BIT = 1 << LOADING_OFFSET;
var DEFAULT_FLAGS = ERROR_BIT;

// src/core.ts
var currentObserver = null;
var currentMask = DEFAULT_FLAGS;
var newSources = null;
var newSourcesIndex = 0;
var newFlags = 0;
function getObserver() {
  return currentObserver;
}
var UNCHANGED = Symbol(0);
var Computation = class extends Owner {
  c = null;
  b = null;
  j;
  v;
  // Used in __DEV__ mode, hopefully removed in production
  D;
  // Using false is an optimization as an alternative to _equals: () => false
  // which could enable more efficient DIRTY notification
  w = isEqual;
  /** Whether the computation is an error or has ancestors that are unresolved */
  m = 0;
  /** Which flags raised by sources are handled, vs. being passed through. */
  u = DEFAULT_FLAGS;
  x = null;
  y = null;
  constructor(initialValue, compute2, options) {
    super(compute2 === null);
    this.v = compute2;
    this.a = compute2 ? STATE_DIRTY : STATE_CLEAN;
    this.j = initialValue;
    if (options?.equals !== void 0) this.w = options.equals;
  }
  B() {
    if (this.v) this.p();
    track(this);
    newFlags |= this.m & ~currentMask;
    if (this.m & ERROR_BIT) {
      throw this.j;
    } else {
      return this.j;
    }
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   */
  read() {
    return this.B();
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   *
   * If the computation has any unresolved ancestors, this function waits for the value to resolve
   * before continuing
   */
  wait() {
    if (this.loading()) {
      throw new NotReadyError();
    }
    return this.B();
  }
  /**
   * Return true if the computation is the value is dependent on an unresolved promise
   * Triggers re-execution of the computation when the loading state changes
   *
   * This is useful especially when effects want to re-execute when a computation's
   * loading state changes
   */
  loading() {
    if (this.y === null) {
      this.y = loadingState(this);
    }
    return this.y.read();
  }
  /**
   * Return true if the computation is the computation threw an error
   * Triggers re-execution of the computation when the error state changes
   */
  error() {
    if (this.x === null) {
      this.x = errorState(this);
    }
    return this.x.read();
  }
  /** Update the computation with a new value. */
  write(value, flags = 0, raw = false) {
    const newValue =
      !raw && typeof value === "function" ? value(this.j) : value;
    const valueChanged =
      newValue !== UNCHANGED &&
      (!!(flags & ERROR_BIT) || this.w === false || !this.w(this.j, newValue));
    if (valueChanged) this.j = newValue;
    const changedFlagsMask = this.m ^ flags,
      changedFlags = changedFlagsMask & flags;
    this.m = flags;
    if (this.b) {
      for (let i = 0; i < this.b.length; i++) {
        if (valueChanged) {
          this.b[i].q(STATE_DIRTY);
        } else if (changedFlagsMask) {
          this.b[i].C(changedFlagsMask, changedFlags);
        }
      }
    }
    return this.j;
  }
  /**
   * Set the current node's state, and recursively mark all of this node's observers as STATE_CHECK
   */
  q(state) {
    if (this.a >= state) return;
    this.a = state;
    if (this.b) {
      for (let i = 0; i < this.b.length; i++) {
        this.b[i].q(STATE_CHECK);
      }
    }
  }
  /**
   * Notify the computation that one of its sources has changed flags.
   *
   * @param mask A bitmask for which flag(s) were changed.
   * @param newFlags The source's new flags, masked to just the changed ones.
   */
  C(mask, newFlags2) {
    if (this.a >= STATE_DIRTY) return;
    if (mask & this.u) {
      this.q(STATE_DIRTY);
      return;
    }
    if (this.a >= STATE_CHECK) return;
    const prevFlags = this.m & mask;
    const deltaFlags = prevFlags ^ newFlags2;
    if (newFlags2 === prevFlags);
    else if (deltaFlags & prevFlags & mask) {
      this.q(STATE_CHECK);
    } else {
      this.m ^= deltaFlags;
      if (this.b) {
        for (let i = 0; i < this.b.length; i++) {
          this.b[i].C(mask, newFlags2);
        }
      }
    }
  }
  z(error) {
    this.write(error, this.m | ERROR_BIT);
  }
  /**
   * This is the core part of the reactivity system, which makes sure that the values are updated
   * before they are read. We've also adapted it to return the loading state of the computation,
   * so that we can propagate that to the computation's observers.
   *
   * This function will ensure that the value and states we read from the computation are up to date
   */
  p() {
    if (this.a === STATE_DISPOSED) {
      throw new Error("Tried to read a disposed computation");
    }
    if (this.a === STATE_CLEAN) {
      return;
    }
    let observerFlags = 0;
    if (this.a === STATE_CHECK) {
      for (let i = 0; i < this.c.length; i++) {
        this.c[i].p();
        observerFlags |= this.c[i].m;
        if (this.a === STATE_DIRTY) {
          break;
        }
      }
    }
    if (this.a === STATE_DIRTY) {
      update(this);
    } else {
      this.write(UNCHANGED, observerFlags);
      this.a = STATE_CLEAN;
    }
  }
  /**
   * Remove ourselves from the owner graph and the computation graph
   */
  t() {
    if (this.a === STATE_DISPOSED) return;
    if (this.c) removeSourceObservers(this, 0);
    super.t();
  }
};
function loadingState(node) {
  const prevOwner = setOwner(node.o);
  const options = void 0;
  const computation = new Computation(
    void 0,
    () => {
      track(node);
      node.p();
      return !!(node.m & LOADING_BIT);
    },
    options,
  );
  computation.u = ERROR_BIT | LOADING_BIT;
  setOwner(prevOwner);
  return computation;
}
function errorState(node) {
  const prevOwner = setOwner(node.o);
  const options = void 0;
  const computation = new Computation(
    void 0,
    () => {
      track(node);
      node.p();
      return !!(node.m & ERROR_BIT);
    },
    options,
  );
  computation.u = ERROR_BIT;
  setOwner(prevOwner);
  return computation;
}
function track(computation) {
  if (currentObserver) {
    if (
      !newSources &&
      currentObserver.c &&
      currentObserver.c[newSourcesIndex] === computation
    ) {
      newSourcesIndex++;
    } else if (!newSources) newSources = [computation];
    else if (computation !== newSources[newSources.length - 1]) {
      newSources.push(computation);
    }
  }
}
function update(node) {
  const prevSources = newSources,
    prevSourcesIndex = newSourcesIndex,
    prevFlags = newFlags;
  newSources = null;
  newSourcesIndex = 0;
  newFlags = 0;
  try {
    node.dispose(false);
    node.emptyDisposal();
    const result = compute(node, node.v, node);
    node.write(result, newFlags, true);
  } catch (error) {
    if (error instanceof NotReadyError) {
      node.write(UNCHANGED, newFlags | LOADING_BIT);
    } else {
      node.z(error);
    }
  } finally {
    if (newSources) {
      if (node.c) removeSourceObservers(node, newSourcesIndex);
      if (node.c && newSourcesIndex > 0) {
        node.c.length = newSourcesIndex + newSources.length;
        for (let i = 0; i < newSources.length; i++) {
          node.c[newSourcesIndex + i] = newSources[i];
        }
      } else {
        node.c = newSources;
      }
      let source;
      for (let i = newSourcesIndex; i < node.c.length; i++) {
        source = node.c[i];
        if (!source.b) source.b = [node];
        else source.b.push(node);
      }
    } else if (node.c && newSourcesIndex < node.c.length) {
      removeSourceObservers(node, newSourcesIndex);
      node.c.length = newSourcesIndex;
    }
    newSources = prevSources;
    newSourcesIndex = prevSourcesIndex;
    newFlags = prevFlags;
    node.a = STATE_CLEAN;
  }
}
function removeSourceObservers(node, index) {
  let source;
  let swap;
  for (let i = index; i < node.c.length; i++) {
    source = node.c[i];
    if (source.b) {
      swap = source.b.indexOf(node);
      source.b[swap] = source.b[source.b.length - 1];
      source.b.pop();
    }
  }
}
function isEqual(a, b) {
  return a === b;
}
function untrack(fn) {
  if (currentObserver === null) return fn();
  return compute(getOwner(), fn, null);
}
function compute(owner, compute2, observer) {
  const prevOwner = setOwner(owner),
    prevObserver = currentObserver,
    prevMask = currentMask;
  currentObserver = observer;
  currentMask = observer?.u ?? DEFAULT_FLAGS;
  try {
    return compute2(observer ? observer.j : void 0);
  } finally {
    setOwner(prevOwner);
    currentObserver = prevObserver;
    currentMask = prevMask;
  }
}

// src/effect.ts
var scheduledEffects = false;
var runningEffects = false;
var renderEffects = [];
var effects = [];
function flushSync() {
  if (!runningEffects) runEffects();
}
function flushEffects() {
  scheduledEffects = true;
  queueMicrotask(runEffects);
}
function runTop(node) {
  const ancestors = [];
  for (let current = node; current !== null; current = current.o) {
    if (current.a !== STATE_CLEAN) {
      ancestors.push(current);
    }
  }
  for (let i = ancestors.length - 1; i >= 0; i--) {
    if (ancestors[i].a !== STATE_DISPOSED) ancestors[i].p();
  }
}
function runEffects() {
  if (!effects.length) {
    scheduledEffects = false;
    return;
  }
  runningEffects = true;
  try {
    for (let i = 0; i < renderEffects.length; i++) {
      if (renderEffects[i].a !== STATE_CLEAN) {
        renderEffects[i].p();
      }
    }
    for (let i = 0; i < renderEffects.length; i++) {
      if (renderEffects[i].modified) {
        renderEffects[i].effect(renderEffects[i].j);
        renderEffects[i].modified = false;
      }
    }
    for (let i = 0; i < effects.length; i++) {
      if (effects[i].a !== STATE_CLEAN) {
        runTop(effects[i]);
      }
    }
  } finally {
    effects = [];
    renderEffects = [];
    scheduledEffects = false;
    runningEffects = false;
  }
}
var Effect = class extends Computation {
  constructor(initialValue, compute2, options) {
    super(initialValue, compute2, options);
    effects.push(this);
    flushEffects();
  }
  q(state) {
    if (this.a >= state) return;
    if (this.a === STATE_CLEAN) {
      effects.push(this);
      if (!scheduledEffects) flushEffects();
    }
    this.a = state;
  }
  write(value) {
    this.j = value;
    return value;
  }
  z(error) {
    this.handleError(error);
  }
};
var RenderEffect = class extends Computation {
  effect;
  modified = false;
  constructor(initialValue, compute2, effect, options) {
    super(initialValue, compute2, options);
    this.effect = effect;
    this.p();
  }
  q(state) {
    if (this.a >= state) return;
    if (this.a === STATE_CLEAN) {
      renderEffects.push(this);
      if (!scheduledEffects) flushEffects();
    }
    this.a = state;
  }
  write(value) {
    this.j = value;
    this.modified = true;
    return value;
  }
  z(error) {
    this.handleError(error);
  }
};

// src/signals.ts
function createSignal(initialValue, options) {
  const node = new Computation(initialValue, null, options);
  return [node.read.bind(node), node.write.bind(node)];
}
function createAsync(fn, initial, options) {
  const lhs = new Computation(void 0, () => {
    const promise = Promise.resolve(fn());
    const signal = new Computation(initial, null, options);
    signal.write(UNCHANGED, LOADING_BIT);
    promise.then(
      (value) => {
        signal.write(value, 0);
      },
      (error) => {
        signal.write(error, ERROR_BIT);
      },
    );
    return signal;
  });
  const rhs = new Computation(void 0, () => lhs.read().wait(), options);
  return () => rhs.wait();
}
function createMemo(compute2, initialValue, options) {
  const node = new Computation(initialValue, compute2, options);
  return node.read.bind(node);
}
function createEffect(effect, initialValue, options) {
  void new Effect(initialValue, effect, void 0);
}
function createRenderEffect(compute2, effect, initialValue, options) {
  void new RenderEffect(initialValue, compute2, effect, void 0);
}
function createRoot(init) {
  const owner = new Owner();
  return compute(
    owner,
    !init.length ? init : () => init(() => owner.dispose()),
    null,
  );
}
function runWithOwner(owner, run) {
  try {
    return compute(owner, run, null);
  } catch (error) {
    owner?.handleError(error);
    return void 0;
  }
}
function catchError(fn, handler) {
  const owner = new Owner();
  owner.g = owner.g ? [handler, ...owner.g] : [handler];
  try {
    compute(owner, fn, null);
  } catch (error) {
    owner.handleError(error);
  }
}

// src/map.ts
function indexArray(list, map, options) {
  return Computation.prototype.read.bind(
    new Computation(
      [],
      updateMap.bind({
        r: new Owner(),
        i: 0,
        A: list,
        h: [],
        s: map,
        d: [],
        e: [],
      }),
      options,
    ),
  );
}
function updateMap() {
  let i = 0,
    newItems = this.A() || [],
    mapper = () => this.s(Computation.prototype.read.bind(this.e[i]), i);
  runWithOwner(this.r, () => {
    if (newItems.length === 0) {
      if (this.i !== 0) {
        this.r.dispose(false);
        this.h = [];
        this.d = [];
        this.i = 0;
        this.e = [];
      }
      return;
    }
    for (i = 0; i < newItems.length; i++) {
      if (i < this.h.length && this.h[i] !== newItems[i]) {
        this.e[i].write(newItems[i]);
      } else if (i >= this.h.length) {
        this.d[i] = compute(
          (this.e[i] = new Computation(newItems[i], null)),
          mapper,
          null,
        );
      }
    }
    for (; i < this.h.length; i++) this.e[i].dispose();
    this.i = this.e.length = newItems.length;
    this.h = newItems.slice(0);
    this.d = this.d.slice(0, this.i);
  });
  return this.d;
}
function mapArray(list, map, options) {
  return Computation.prototype.read.bind(
    new Computation(
      [],
      updateKeyedMap.bind({
        r: new Owner(),
        i: 0,
        A: list,
        h: [],
        s: map,
        d: [],
        e: [],
      }),
      options,
    ),
  );
}
function updateKeyedMap() {
  const newItems = this.A() || [],
    indexed = this.s.length > 1;
  runWithOwner(this.r, () => {
    let newLen = newItems.length,
      i,
      j,
      mapper = indexed
        ? () => this.s(newItems[j], Computation.prototype.read.bind(this.e[j]))
        : () => this.s(newItems[j]);
    if (newLen === 0) {
      if (this.i !== 0) {
        this.r.dispose(false);
        this.e = [];
        this.h = [];
        this.d = [];
        this.i = 0;
      }
    } else if (this.i === 0) {
      this.d = new Array(newLen);
      for (j = 0; j < newLen; j++) {
        this.h[j] = newItems[j];
        this.d[j] = compute(
          (this.e[j] = new Computation(j, null)),
          mapper,
          null,
        );
      }
      this.i = newLen;
    } else {
      let start,
        end,
        newEnd,
        item,
        newIndices,
        newIndicesNext,
        temp = new Array(newLen),
        tempNodes = new Array(newLen);
      for (
        start = 0, end = Math.min(this.i, newLen);
        start < end && this.h[start] === newItems[start];
        start++
      );
      for (
        end = this.i - 1, newEnd = newLen - 1;
        end >= start && newEnd >= start && this.h[end] === newItems[newEnd];
        end--, newEnd--
      ) {
        temp[newEnd] = this.d[end];
        tempNodes[newEnd] = this.e[end];
      }
      newIndices = /* @__PURE__ */ new Map();
      newIndicesNext = new Array(newEnd + 1);
      for (j = newEnd; j >= start; j--) {
        item = newItems[j];
        i = newIndices.get(item);
        newIndicesNext[j] = i === void 0 ? -1 : i;
        newIndices.set(item, j);
      }
      for (i = start; i <= end; i++) {
        item = this.h[i];
        j = newIndices.get(item);
        if (j !== void 0 && j !== -1) {
          temp[j] = this.d[i];
          tempNodes[j] = this.e[i];
          j = newIndicesNext[j];
          newIndices.set(item, j);
        } else this.e[i].dispose();
      }
      for (j = start; j < newLen; j++) {
        if (j in temp) {
          this.d[j] = temp[j];
          this.e[j] = tempNodes[j];
          this.e[j].write(j);
        } else {
          this.d[j] = compute(
            (this.e[j] = new Computation(j, null)),
            mapper,
            null,
          );
        }
      }
      this.d = this.d.slice(0, (this.i = newLen));
      this.h = newItems.slice(0);
    }
  });
  return this.d;
}

// src/selector.ts
function createSelector(source, options) {
  let prevSource,
    subs = /* @__PURE__ */ new Map(),
    equals = options?.equals ?? isEqual;
  const node = new Effect(
    void 0,
    () => {
      const newSource = source();
      for (const [key, val] of subs) {
        if (equals(key, newSource) !== equals(key, prevSource)) {
          for (const c of val.values()) {
            c.q(STATE_DIRTY);
          }
        }
      }
      return (prevSource = newSource);
    },
    void 0,
  );
  return function observeSelector(key) {
    const observer = getObserver();
    if (observer) {
      let l;
      if ((l = subs.get(key))) l.add(observer);
      else subs.set(key, (l = /* @__PURE__ */ new Set([observer])));
      onCleanup(() => {
        l.delete(observer);
        !l.size && subs.delete(key);
      });
    }
    return equals(key, node.read());
  };
}

// src/store.ts
var $RAW = Symbol(0);
var $TRACK = Symbol(0);
var $PROXY = Symbol(0);
var PROXIES = /* @__PURE__ */ new WeakMap();
var NODES = [/* @__PURE__ */ new WeakMap(), /* @__PURE__ */ new WeakMap()];
function wrap(value) {
  let p = PROXIES.get(value);
  if (!p) PROXIES.set(value, (p = new Proxy(value, proxyTraps)));
  return p;
}
function isWrappable(obj) {
  let proto;
  return (
    obj != null &&
    typeof obj === "object" &&
    (PROXIES.has(obj) ||
      !(proto = Object.getPrototypeOf(obj)) ||
      proto === Object.prototype ||
      Array.isArray(obj))
  );
}
function unwrap(item, set = /* @__PURE__ */ new Set()) {
  let result, unwrapped, v, prop;
  if ((result = item != null && item[$RAW])) return result;
  if (!isWrappable(item) || set.has(item)) return item;
  if (Array.isArray(item)) {
    if (Object.isFrozen(item)) item = item.slice(0);
    else set.add(item);
    for (let i = 0, l = item.length; i < l; i++) {
      v = item[i];
      if ((unwrapped = unwrap(v, set)) !== v) item[i] = unwrapped;
    }
  } else {
    if (Object.isFrozen(item)) item = Object.assign({}, item);
    else set.add(item);
    const keys = Object.keys(item);
    for (let i = 0, l = keys.length; i < l; i++) {
      prop = keys[i];
      const desc = Object.getOwnPropertyDescriptor(item, prop);
      if (desc.get) continue;
      v = item[prop];
      if ((unwrapped = unwrap(v, set)) !== v) item[prop] = unwrapped;
    }
  }
  return item;
}
function getNodes(target, type) {
  let nodes = NODES[type].get(target);
  if (!nodes)
    NODES[type].set(target, (nodes = /* @__PURE__ */ Object.create(null)));
  return nodes;
}
function getNode(nodes, property, value) {
  if (nodes[property]) return nodes[property];
  return (nodes[property] = new Computation(value, null, {
    equals: false,
  }));
}
function proxyDescriptor(target, property) {
  const desc = Reflect.getOwnPropertyDescriptor(target, property);
  if (!desc || desc.get || !desc.configurable || property === $PROXY)
    return desc;
  delete desc.value;
  delete desc.writable;
  desc.get = () => PROXIES.get(target)[property];
  return desc;
}
function trackSelf(target) {
  getObserver() && getNode(getNodes(target, 0), $TRACK).read();
}
function ownKeys(target) {
  trackSelf(target);
  return Reflect.ownKeys(target);
}
var Writing = false;
var proxyTraps = {
  get(target, property, receiver) {
    if (property === $RAW) return target;
    if (property === $PROXY) return receiver;
    if (property === $TRACK) {
      trackSelf(target);
      return receiver;
    }
    const desc = Object.getOwnPropertyDescriptor(target, property);
    if (desc && desc.get) return desc.get.call(receiver);
    const nodes = getNodes(target, 0);
    const tracked = nodes[property];
    let value = tracked ? nodes[property].read() : target[property];
    if (
      !tracked &&
      getObserver() &&
      (typeof value !== "function" || target.hasOwnProperty(property))
    )
      value = getNode(nodes, property, value).read();
    return isWrappable(value) ? wrap(value) : value;
  },
  has(target, property) {
    if (
      property === $RAW ||
      property === $PROXY ||
      property === $TRACK ||
      property === "__proto__"
    )
      return true;
    getObserver() && getNode(getNodes(target, 1), property).read();
    return property in target;
  },
  set(target, property, value) {
    Writing && setProperty(target, property, unwrap(value));
    return true;
  },
  deleteProperty(target, property) {
    Writing && setProperty(target, property, void 0, true);
    return true;
  },
  ownKeys,
  getOwnPropertyDescriptor: proxyDescriptor,
};
function setProperty(state, property, value, deleting = false) {
  if (!deleting && state[property] === value) return;
  const prev = state[property];
  const len = state.length;
  if (deleting) delete state[property];
  else state[property] = value;
  const nodes = getNodes(state, 0);
  let node;
  if ((node = getNode(nodes, property, prev))) node.write(value);
  if (Array.isArray(state) && state.length !== len)
    (node = getNode(nodes, "length", len)) && node.write(state.length);
  (node = nodes[$TRACK]) && node.write(void 0);
}
function createStore(store) {
  const unwrappedStore = unwrap(store);
  const wrappedStore = wrap(unwrappedStore);
  const setStore = (fn) => {
    try {
      Writing = true;
      fn(wrappedStore);
    } finally {
      Writing = false;
    }
  };
  return [wrappedStore, setStore];
}

export {
  Computation,
  ContextNotFoundError,
  Effect,
  NoOwnerError,
  NotReadyError,
  Owner,
  RenderEffect,
  catchError,
  compute,
  createAsync,
  createContext,
  createEffect,
  createMemo,
  createRenderEffect,
  createRoot,
  createSelector,
  createSignal,
  createStore,
  flushSync,
  getContext,
  getObserver,
  getOwner,
  hasContext,
  indexArray,
  isEqual,
  isWrappable,
  mapArray,
  onCleanup,
  runWithOwner,
  setContext,
  setOwner,
  untrack,
  unwrap,
};
