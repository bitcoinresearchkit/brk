// src/core/error.ts
var NotReadyError = class extends Error {
};
var NoOwnerError = class extends Error {
  constructor() {
    super(
      ""
    );
  }
};
var ContextNotFoundError = class extends Error {
  constructor() {
    super(
      ""
    );
  }
};

// src/utils.ts
function isUndefined(value) {
  return typeof value === "undefined";
}

// src/core/constants.ts
var STATE_CLEAN = 0;
var STATE_CHECK = 1;
var STATE_DIRTY = 2;
var STATE_DISPOSED = 3;

// src/core/owner.ts
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
  p = null;
  i = null;
  m = null;
  a = STATE_CLEAN;
  f = null;
  j = defaultContext;
  g = null;
  constructor(signal = false) {
    if (currentOwner && !signal)
      currentOwner.append(this);
  }
  append(child) {
    child.p = this;
    child.m = this;
    if (this.i)
      this.i.m = child;
    child.i = this.i;
    this.i = child;
    if (child.j !== this.j) {
      child.j = { ...this.j, ...child.j };
    }
    if (this.g) {
      child.g = !child.g ? this.g : [...child.g, ...this.g];
    }
  }
  dispose(self = true) {
    if (this.a === STATE_DISPOSED)
      return;
    let head = self ? this.m || this.p : this, current = this.i, next = null;
    while (current && current.p === this) {
      current.dispose(true);
      current.t();
      next = current.i;
      current.i = null;
      current = next;
    }
    if (self)
      this.t();
    if (current)
      current.m = !self ? this : this.m;
    if (head)
      head.i = current;
  }
  t() {
    if (this.m)
      this.m.i = null;
    this.p = null;
    this.m = null;
    this.j = defaultContext;
    this.g = null;
    this.a = STATE_DISPOSED;
    this.emptyDisposal();
  }
  emptyDisposal() {
    if (!this.f)
      return;
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
    if (!this.g)
      throw error;
    let i = 0, len = this.g.length;
    for (i = 0; i < len; i++) {
      try {
        this.g[i](error);
        break;
      } catch (e) {
        error = e;
      }
    }
    if (i === len)
      throw error;
  }
};
function createContext(defaultValue, description) {
  return { id: Symbol(description), defaultValue };
}
function getContext(context, owner = currentOwner) {
  if (!owner) {
    throw new NoOwnerError();
  }
  const value = hasContext(context, owner) ? owner.j[context.id] : context.defaultValue;
  if (isUndefined(value)) {
    throw new ContextNotFoundError();
  }
  return value;
}
function setContext(context, value, owner = currentOwner) {
  if (!owner) {
    throw new NoOwnerError();
  }
  owner.j = {
    ...owner.j,
    [context.id]: isUndefined(value) ? context.defaultValue : value
  };
}
function hasContext(context, owner = currentOwner) {
  return !isUndefined(owner?.j[context.id]);
}
function onCleanup(disposable) {
  if (!currentOwner)
    return;
  const node = currentOwner;
  if (!node.f) {
    node.f = disposable;
  } else if (Array.isArray(node.f)) {
    node.f.push(disposable);
  } else {
    node.f = [node.f, disposable];
  }
}

// src/core/flags.ts
var ERROR_OFFSET = 0;
var ERROR_BIT = 1 << ERROR_OFFSET;
var LOADING_OFFSET = 1;
var LOADING_BIT = 1 << LOADING_OFFSET;
var DEFAULT_FLAGS = ERROR_BIT;

// src/core/scheduler.ts
var scheduled = false;
var runningScheduled = false;
var Computations = [];
var RenderEffects = [];
var Effects = [];
function flushSync() {
  if (!runningScheduled)
    runScheduled();
}
function flushQueue() {
  if (scheduled)
    return;
  scheduled = true;
  queueMicrotask(runScheduled);
}
function runTop(node) {
  const ancestors = [];
  for (let current = node; current !== null; current = current.p) {
    if (current.a !== STATE_CLEAN) {
      ancestors.push(current);
    }
  }
  for (let i = ancestors.length - 1; i >= 0; i--) {
    if (ancestors[i].a !== STATE_DISPOSED)
      ancestors[i].q();
  }
}
function runScheduled() {
  if (!Effects.length && !RenderEffects.length && !Computations.length) {
    scheduled = false;
    return;
  }
  runningScheduled = true;
  try {
    runPureQueue(Computations);
    runPureQueue(RenderEffects);
    runPureQueue(Effects);
  } finally {
    const renderEffects = RenderEffects;
    const effects = Effects;
    Computations = [];
    Effects = [];
    RenderEffects = [];
    scheduled = false;
    runningScheduled = false;
    incrementClock();
    runEffectQueue(renderEffects);
    runEffectQueue(effects);
  }
}
function runPureQueue(queue) {
  for (let i = 0; i < queue.length; i++) {
    if (queue[i].a !== STATE_CLEAN)
      runTop(queue[i]);
  }
}
function runEffectQueue(queue) {
  for (let i = 0; i < queue.length; i++) {
    if (queue[i].x && queue[i].a !== STATE_DISPOSED) {
      queue[i].y(queue[i].d, queue[i].v);
      queue[i].x = false;
      queue[i].v = queue[i].d;
    }
  }
}

// src/core/core.ts
var currentObserver = null;
var currentMask = DEFAULT_FLAGS;
var newSources = null;
var newSourcesIndex = 0;
var newFlags = 0;
var clock = 0;
var syncResolve = false;
var updateCheck = null;
function getObserver() {
  return currentObserver;
}
function incrementClock() {
  clock++;
}
var UNCHANGED = Symbol(0);
var Computation2 = class extends Owner {
  b = null;
  c = null;
  d;
  z;
  // Used in __DEV__ mode, hopefully removed in production
  L;
  // Using false is an optimization as an alternative to _equals: () => false
  // which could enable more efficient DIRTY notification
  A = isEqual;
  F;
  /** Whether the computation is an error or has ancestors that are unresolved */
  k = 0;
  /** Which flags raised by sources are handled, vs. being passed through. */
  w = DEFAULT_FLAGS;
  B = null;
  C = null;
  D = -1;
  constructor(initialValue, compute2, options) {
    super(compute2 === null);
    this.z = compute2;
    this.a = compute2 ? STATE_DIRTY : STATE_CLEAN;
    this.d = initialValue;
    if (options?.equals !== void 0)
      this.A = options.equals;
    if (options?.unobserved)
      this.F = options?.unobserved;
  }
  G() {
    if (this.z)
      this.q();
    if (!this.b || this.b.length)
      track(this);
    newFlags |= this.k & ~currentMask;
    if (this.k & ERROR_BIT) {
      throw this.d;
    } else {
      return this.d;
    }
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   */
  read() {
    return this.G();
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   *
   * If the computation has any unresolved ancestors, this function waits for the value to resolve
   * before continuing
   */
  wait() {
    if (!syncResolve && this.loading()) {
      throw new NotReadyError();
    }
    return this.G();
  }
  /**
   * Return true if the computation is the value is dependent on an unresolved promise
   * Triggers re-execution of the computation when the loading state changes
   *
   * This is useful especially when effects want to re-execute when a computation's
   * loading state changes
   */
  loading() {
    if (this.C === null) {
      this.C = loadingState(this);
    }
    return this.C.read();
  }
  /**
   * Return true if the computation is the computation threw an error
   * Triggers re-execution of the computation when the error state changes
   */
  error() {
    if (this.B === null) {
      this.B = errorState(this);
    }
    return this.B.read();
  }
  /** Update the computation with a new value. */
  write(value, flags = 0, raw = false) {
    const newValue = !raw && typeof value === "function" ? value(this.d) : value;
    const valueChanged = newValue !== UNCHANGED && (!!(flags & ERROR_BIT) || this.A === false || !this.A(this.d, newValue));
    if (valueChanged)
      this.d = newValue;
    const changedFlagsMask = this.k ^ flags, changedFlags = changedFlagsMask & flags;
    this.k = flags;
    this.D = clock + 1;
    if (this.c) {
      for (let i = 0; i < this.c.length; i++) {
        if (valueChanged) {
          this.c[i].n(STATE_DIRTY);
        } else if (changedFlagsMask) {
          this.c[i].H(changedFlagsMask, changedFlags);
        }
      }
    }
    return this.d;
  }
  /**
   * Set the current node's state, and recursively mark all of this node's observers as STATE_CHECK
   */
  n(state) {
    if (this.a >= state)
      return;
    this.a = state;
    if (this.c) {
      for (let i = 0; i < this.c.length; i++) {
        this.c[i].n(STATE_CHECK);
      }
    }
  }
  /**
   * Notify the computation that one of its sources has changed flags.
   *
   * @param mask A bitmask for which flag(s) were changed.
   * @param newFlags The source's new flags, masked to just the changed ones.
   */
  H(mask, newFlags2) {
    if (this.a >= STATE_DIRTY)
      return;
    if (mask & this.w) {
      this.n(STATE_DIRTY);
      return;
    }
    if (this.a >= STATE_CHECK)
      return;
    const prevFlags = this.k & mask;
    const deltaFlags = prevFlags ^ newFlags2;
    if (newFlags2 === prevFlags) ; else if (deltaFlags & prevFlags & mask) {
      this.n(STATE_CHECK);
    } else {
      this.k ^= deltaFlags;
      if (this.c) {
        for (let i = 0; i < this.c.length; i++) {
          this.c[i].H(mask, newFlags2);
        }
      }
    }
  }
  I(error) {
    this.write(error, this.k | ERROR_BIT);
  }
  /**
   * This is the core part of the reactivity system, which makes sure that the values are updated
   * before they are read. We've also adapted it to return the loading state of the computation,
   * so that we can propagate that to the computation's observers.
   *
   * This function will ensure that the value and states we read from the computation are up to date
   */
  q() {
    if (this.a === STATE_DISPOSED) {
      throw new Error("Tried to read a disposed computation");
    }
    if (this.a === STATE_CLEAN) {
      return;
    }
    let observerFlags = 0;
    if (this.a === STATE_CHECK) {
      for (let i = 0; i < this.b.length; i++) {
        this.b[i].q();
        observerFlags |= this.b[i].k;
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
    if (this.a === STATE_DISPOSED)
      return;
    if (this.b)
      removeSourceObservers(this, 0);
    super.t();
  }
};
function loadingState(node) {
  const prevOwner = setOwner(node.p);
  const options = void 0;
  const computation = new Computation2(
    void 0,
    () => {
      track(node);
      node.q();
      return !!(node.k & LOADING_BIT);
    },
    options
  );
  computation.w = ERROR_BIT | LOADING_BIT;
  setOwner(prevOwner);
  return computation;
}
function errorState(node) {
  const prevOwner = setOwner(node.p);
  const options = void 0;
  const computation = new Computation2(
    void 0,
    () => {
      track(node);
      node.q();
      return !!(node.k & ERROR_BIT);
    },
    options
  );
  computation.w = ERROR_BIT;
  setOwner(prevOwner);
  return computation;
}
function track(computation) {
  if (currentObserver) {
    if (!newSources && currentObserver.b && currentObserver.b[newSourcesIndex] === computation) {
      newSourcesIndex++;
    } else if (!newSources)
      newSources = [computation];
    else if (computation !== newSources[newSources.length - 1]) {
      newSources.push(computation);
    }
    if (updateCheck) {
      updateCheck.d = computation.D > currentObserver.D;
    }
  }
}
function update(node) {
  const prevSources = newSources, prevSourcesIndex = newSourcesIndex, prevFlags = newFlags;
  newSources = null;
  newSourcesIndex = 0;
  newFlags = 0;
  try {
    node.dispose(false);
    node.emptyDisposal();
    const result = compute(node, node.z, node);
    node.write(result, newFlags, true);
  } catch (error) {
    if (error instanceof NotReadyError) {
      node.write(UNCHANGED, newFlags | LOADING_BIT);
    } else {
      node.I(error);
    }
  } finally {
    if (newSources) {
      if (node.b)
        removeSourceObservers(node, newSourcesIndex);
      if (node.b && newSourcesIndex > 0) {
        node.b.length = newSourcesIndex + newSources.length;
        for (let i = 0; i < newSources.length; i++) {
          node.b[newSourcesIndex + i] = newSources[i];
        }
      } else {
        node.b = newSources;
      }
      let source;
      for (let i = newSourcesIndex; i < node.b.length; i++) {
        source = node.b[i];
        if (!source.c)
          source.c = [node];
        else
          source.c.push(node);
      }
    } else if (node.b && newSourcesIndex < node.b.length) {
      removeSourceObservers(node, newSourcesIndex);
      node.b.length = newSourcesIndex;
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
  for (let i = index; i < node.b.length; i++) {
    source = node.b[i];
    if (source.c) {
      swap = source.c.indexOf(node);
      source.c[swap] = source.c[source.c.length - 1];
      source.c.pop();
      if (!source.c.length)
        source.F?.();
    }
  }
}
function isEqual(a, b) {
  return a === b;
}
function untrack(fn) {
  if (currentObserver === null)
    return fn();
  return compute(getOwner(), fn, null);
}
function hasUpdated(fn) {
  const current = updateCheck;
  updateCheck = { d: false };
  try {
    fn();
    return updateCheck.d;
  } finally {
    updateCheck = current;
  }
}
function isPending(fn) {
  try {
    fn();
    return false;
  } catch (e) {
    return e instanceof NotReadyError;
  }
}
function latest(fn) {
  const prevFlags = newFlags;
  syncResolve = true;
  try {
    return fn();
  } catch {
  } finally {
    newFlags = prevFlags;
    syncResolve = false;
  }
}
function compute(owner, compute2, observer) {
  const prevOwner = setOwner(owner), prevObserver = currentObserver, prevMask = currentMask;
  currentObserver = observer;
  currentMask = observer?.w ?? DEFAULT_FLAGS;
  try {
    return compute2(observer ? observer.d : void 0);
  } finally {
    setOwner(prevOwner);
    currentObserver = prevObserver;
    currentMask = prevMask;
  }
}
var EagerComputation = class extends Computation2 {
  constructor(initialValue, compute2, options) {
    super(initialValue, compute2, options);
    this.q();
    Computations.push(this);
  }
  n(state) {
    if (this.a >= state)
      return;
    if (this.a === STATE_CLEAN) {
      Computations.push(this);
      flushQueue();
    }
    super.n(state);
  }
};

// src/core/effect.ts
var BaseEffect = class extends Computation2 {
  y;
  x = false;
  v;
  constructor(initialValue, compute2, effect, options) {
    super(initialValue, compute2, options);
    this.y = effect;
    this.v = initialValue;
  }
  write(value) {
    if (value === UNCHANGED)
      return this.d;
    this.d = value;
    this.x = true;
    return value;
  }
  I(error) {
    this.handleError(error);
  }
  t() {
    this.y = void 0;
    this.v = void 0;
    super.t();
  }
};
var Effect = class extends BaseEffect {
  constructor(initialValue, compute2, effect, options) {
    super(initialValue, compute2, effect, options);
    Effects.push(this);
    flushQueue();
  }
  n(state) {
    if (this.a >= state)
      return;
    if (this.a === STATE_CLEAN) {
      Effects.push(this);
      flushQueue();
    }
    this.a = state;
  }
};
var RenderEffect = class extends BaseEffect {
  constructor(initialValue, compute2, effect, options) {
    super(initialValue, compute2, effect, options);
    this.q();
    RenderEffects.push(this);
  }
  n(state) {
    if (this.a >= state)
      return;
    if (this.a === STATE_CLEAN) {
      RenderEffects.push(this);
      flushQueue();
    }
    this.a = state;
  }
};

// src/signals.ts
function createSignal(first, second, third) {
  if (typeof first === "function") {
    const memo = createMemo((p) => {
      const node2 = new Computation2(
        first(p ? untrack(p[0]) : second),
        null,
        third
      );
      return [node2.read.bind(node2), node2.write.bind(node2)];
    });
    return [() => memo()[0](), (value) => memo()[1](value)];
  }
  const node = new Computation2(first, null, second);
  return [node.read.bind(node), node.write.bind(node)];
}
function createAsync(fn, initial, options) {
  const lhs = new EagerComputation(
    {
      d: initial
    },
    (p) => {
      const value = p?.d;
      const source = fn(value);
      const isPromise = source instanceof Promise;
      const iterator = source[Symbol.asyncIterator];
      if (!isPromise && !iterator) {
        return {
          wait() {
            return source;
          },
          d: source
        };
      }
      const signal = new Computation2(value, null, options);
      signal.write(UNCHANGED, LOADING_BIT);
      if (isPromise) {
        source.then(
          (value2) => {
            signal.write(value2, 0);
          },
          (error) => {
            signal.write(error, ERROR_BIT);
          }
        );
      } else {
        let abort = false;
        onCleanup(() => abort = true);
        (async () => {
          try {
            for await (let value2 of source) {
              if (abort)
                return;
              signal.write(value2, 0);
            }
          } catch (error) {
            signal.write(error, ERROR_BIT);
          }
        })();
      }
      return signal;
    }
  );
  return () => lhs.wait().wait();
}
function createMemo(compute2, initialValue, options) {
  let node = new Computation2(initialValue, compute2, options);
  let value;
  return () => {
    if (node) {
      value = node.wait();
      if (!node.b?.length)
        node = void 0;
    }
    return value;
  };
}
function createEffect(compute2, effect, initialValue, options) {
  void new Effect(
    initialValue,
    compute2,
    effect,
    void 0
  );
}
function createRenderEffect(compute2, effect, initialValue, options) {
  void new RenderEffect(
    initialValue,
    compute2,
    effect,
    void 0
  );
}
function createRoot(init) {
  const owner = new Owner();
  return compute(owner, !init.length ? init : () => init(() => owner.dispose()), null);
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
function mapArray(list, map, options) {
  const keyFn = typeof options?.keyed === "function" ? options.keyed : void 0;
  return Computation2.prototype.read.bind(
    new Computation2(
      [],
      updateKeyedMap.bind({
        J: new Owner(),
        r: 0,
        K: list,
        o: [],
        E: map,
        l: [],
        s: [],
        u: keyFn,
        e: keyFn || options?.keyed === false ? [] : void 0,
        h: map.length > 1 ? [] : void 0
      }),
      options
    )
  );
}
function updateKeyedMap() {
  const newItems = this.K() || [];
  runWithOwner(this.J, () => {
    let newLen = newItems.length, i, j, mapper = this.e ? () => {
      this.e[j] = new Computation2(newItems[j], null);
      this.h[j] = new Computation2(j, null);
      return this.E(
        Computation2.prototype.read.bind(this.e[j]),
        Computation2.prototype.read.bind(this.h[j])
      );
    } : this.h ? () => {
      const item = newItems[j];
      this.h[j] = new Computation2(j, null);
      return this.E(() => item, Computation2.prototype.read.bind(this.h[j]));
    } : () => {
      const item = newItems[j];
      return this.E(() => item);
    };
    if (newLen === 0) {
      if (this.r !== 0) {
        this.J.dispose(false);
        this.s = [];
        this.o = [];
        this.l = [];
        this.r = 0;
        this.e && (this.e = []);
        this.h && (this.h = []);
      }
    } else if (this.r === 0) {
      this.l = new Array(newLen);
      for (j = 0; j < newLen; j++) {
        this.o[j] = newItems[j];
        this.l[j] = compute(this.s[j] = new Owner(), mapper, null);
      }
      this.r = newLen;
    } else {
      let start, end, newEnd, item, key, newIndices, newIndicesNext, temp = new Array(newLen), tempNodes = new Array(newLen), tempRows = this.e ? new Array(newLen) : void 0, tempIndexes = this.h ? new Array(newLen) : void 0;
      for (start = 0, end = Math.min(this.r, newLen); start < end && (this.o[start] === newItems[start] || this.e && compare(this.u, this.o[start], newItems[start])); start++) {
        if (this.e)
          this.e[start].write(newItems[start]);
      }
      for (end = this.r - 1, newEnd = newLen - 1; end >= start && newEnd >= start && (this.o[end] === newItems[newEnd] || this.e && compare(this.u, this.o[end], newItems[newEnd])); end--, newEnd--) {
        temp[newEnd] = this.l[end];
        tempNodes[newEnd] = this.s[end];
        tempRows && (tempRows[newEnd] = this.e[end]);
        tempIndexes && (tempIndexes[newEnd] = this.h[end]);
      }
      newIndices = /* @__PURE__ */ new Map();
      newIndicesNext = new Array(newEnd + 1);
      for (j = newEnd; j >= start; j--) {
        item = newItems[j];
        key = this.u ? this.u(item) : item;
        i = newIndices.get(key);
        newIndicesNext[j] = i === void 0 ? -1 : i;
        newIndices.set(key, j);
      }
      for (i = start; i <= end; i++) {
        item = this.o[i];
        key = this.u ? this.u(item) : item;
        j = newIndices.get(key);
        if (j !== void 0 && j !== -1) {
          temp[j] = this.l[i];
          tempNodes[j] = this.s[i];
          tempRows && (tempRows[j] = this.e[i]);
          tempIndexes && (tempIndexes[j] = this.h[i]);
          j = newIndicesNext[j];
          newIndices.set(key, j);
        } else
          this.s[i].dispose();
      }
      for (j = start; j < newLen; j++) {
        if (j in temp) {
          this.l[j] = temp[j];
          this.s[j] = tempNodes[j];
          if (tempRows) {
            this.e[j] = tempRows[j];
            this.e[j].write(newItems[j]);
          }
          if (tempIndexes) {
            this.h[j] = tempIndexes[j];
            this.h[j].write(j);
          }
        } else {
          this.l[j] = compute(this.s[j] = new Owner(), mapper, null);
        }
      }
      this.l = this.l.slice(0, this.r = newLen);
      this.o = newItems.slice(0);
    }
  });
  return this.l;
}
function compare(key, a, b) {
  return key ? key(a) === key(b) : true;
}

export { Computation2 as Computation, ContextNotFoundError, NoOwnerError, NotReadyError, Owner, catchError, createAsync, createContext, createEffect, createMemo, createRenderEffect, createRoot, createSignal, flushSync, getContext, getObserver, getOwner, hasContext, hasUpdated, isEqual, isPending, latest, mapArray, onCleanup, runWithOwner, setContext, untrack };
