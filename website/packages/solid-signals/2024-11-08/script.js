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
  k = null;
  g = null;
  j = null;
  a = STATE_CLEAN;
  e = null;
  h = defaultContext;
  f = null;
  constructor(signal = false) {
    if (currentOwner && !signal)
      currentOwner.append(this);
  }
  append(child) {
    child.k = this;
    child.j = this;
    if (this.g)
      this.g.j = child;
    child.g = this.g;
    this.g = child;
    if (child.h !== this.h) {
      child.h = { ...this.h, ...child.h };
    }
    if (this.f) {
      child.f = !child.f ? this.f : [...child.f, ...this.f];
    }
  }
  dispose(self = true) {
    if (this.a === STATE_DISPOSED)
      return;
    let head = self ? this.j || this.k : this, current = this.g, next = null;
    while (current && current.k === this) {
      current.dispose(true);
      current.n();
      next = current.g;
      current.g = null;
      current = next;
    }
    if (self)
      this.n();
    if (current)
      current.j = !self ? this : this.j;
    if (head)
      head.g = current;
  }
  n() {
    if (this.j)
      this.j.g = null;
    this.k = null;
    this.j = null;
    this.h = defaultContext;
    this.f = null;
    this.a = STATE_DISPOSED;
    this.emptyDisposal();
  }
  emptyDisposal() {
    if (!this.e)
      return;
    if (Array.isArray(this.e)) {
      for (let i = 0; i < this.e.length; i++) {
        const callable = this.e[i];
        callable.call(callable);
      }
    } else {
      this.e.call(this.e);
    }
    this.e = null;
  }
  handleError(error) {
    if (!this.f)
      throw error;
    let i = 0, len = this.f.length;
    for (i = 0; i < len; i++) {
      try {
        this.f[i](error);
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
  const value = hasContext(context, owner) ? owner.h[context.id] : context.defaultValue;
  if (isUndefined(value)) {
    throw new ContextNotFoundError();
  }
  return value;
}
function setContext(context, value, owner = currentOwner) {
  if (!owner) {
    throw new NoOwnerError();
  }
  owner.h = {
    ...owner.h,
    [context.id]: isUndefined(value) ? context.defaultValue : value
  };
}
function hasContext(context, owner = currentOwner) {
  return !isUndefined(owner?.h[context.id]);
}
function onCleanup(disposable) {
  if (!currentOwner)
    return;
  const node = currentOwner;
  if (!node.e) {
    node.e = disposable;
  } else if (Array.isArray(node.e)) {
    node.e.push(disposable);
  } else {
    node.e = [node.e, disposable];
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
  for (let current = node; current !== null; current = current.k) {
    if (current.a !== STATE_CLEAN) {
      ancestors.push(current);
    }
  }
  for (let i = ancestors.length - 1; i >= 0; i--) {
    if (ancestors[i].a !== STATE_DISPOSED)
      ancestors[i].l();
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
    if (queue[i].q && queue[i].a !== STATE_DISPOSED) {
      queue[i].r(queue[i].d, queue[i].o);
      queue[i].q = false;
      queue[i].o = queue[i].d;
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
  s;
  // Used in __DEV__ mode, hopefully removed in production
  C;
  // Using false is an optimization as an alternative to _equals: () => false
  // which could enable more efficient DIRTY notification
  t = isEqual;
  y;
  /** Whether the computation is an error or has ancestors that are unresolved */
  i = 0;
  /** Which flags raised by sources are handled, vs. being passed through. */
  p = DEFAULT_FLAGS;
  u = null;
  v = null;
  w = -1;
  constructor(initialValue, compute2, options) {
    super(compute2 === null);
    this.s = compute2;
    this.a = compute2 ? STATE_DIRTY : STATE_CLEAN;
    this.d = initialValue;
    if (options?.equals !== void 0)
      this.t = options.equals;
    if (options?.unobserved)
      this.y = options?.unobserved;
  }
  z() {
    if (this.s)
      this.l();
    if (!this.b || this.b.length)
      track(this);
    newFlags |= this.i & ~currentMask;
    if (this.i & ERROR_BIT) {
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
    return this.z();
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
    return this.z();
  }
  /**
   * Return true if the computation is the value is dependent on an unresolved promise
   * Triggers re-execution of the computation when the loading state changes
   *
   * This is useful especially when effects want to re-execute when a computation's
   * loading state changes
   */
  loading() {
    if (this.v === null) {
      this.v = loadingState(this);
    }
    return this.v.read();
  }
  /**
   * Return true if the computation is the computation threw an error
   * Triggers re-execution of the computation when the error state changes
   */
  error() {
    if (this.u === null) {
      this.u = errorState(this);
    }
    return this.u.read();
  }
  /** Update the computation with a new value. */
  write(value, flags = 0, raw = false) {
    const newValue = !raw && typeof value === "function" ? value(this.d) : value;
    const valueChanged = newValue !== UNCHANGED && (!!(flags & ERROR_BIT) || this.t === false || !this.t(this.d, newValue));
    if (valueChanged)
      this.d = newValue;
    const changedFlagsMask = this.i ^ flags, changedFlags = changedFlagsMask & flags;
    this.i = flags;
    this.w = clock + 1;
    if (this.c) {
      for (let i = 0; i < this.c.length; i++) {
        if (valueChanged) {
          this.c[i].m(STATE_DIRTY);
        } else if (changedFlagsMask) {
          this.c[i].A(changedFlagsMask, changedFlags);
        }
      }
    }
    return this.d;
  }
  /**
   * Set the current node's state, and recursively mark all of this node's observers as STATE_CHECK
   */
  m(state) {
    if (this.a >= state)
      return;
    this.a = state;
    if (this.c) {
      for (let i = 0; i < this.c.length; i++) {
        this.c[i].m(STATE_CHECK);
      }
    }
  }
  /**
   * Notify the computation that one of its sources has changed flags.
   *
   * @param mask A bitmask for which flag(s) were changed.
   * @param newFlags The source's new flags, masked to just the changed ones.
   */
  A(mask, newFlags2) {
    if (this.a >= STATE_DIRTY)
      return;
    if (mask & this.p) {
      this.m(STATE_DIRTY);
      return;
    }
    if (this.a >= STATE_CHECK)
      return;
    const prevFlags = this.i & mask;
    const deltaFlags = prevFlags ^ newFlags2;
    if (newFlags2 === prevFlags) ; else if (deltaFlags & prevFlags & mask) {
      this.m(STATE_CHECK);
    } else {
      this.i ^= deltaFlags;
      if (this.c) {
        for (let i = 0; i < this.c.length; i++) {
          this.c[i].A(mask, newFlags2);
        }
      }
    }
  }
  B(error) {
    this.write(error, this.i | ERROR_BIT);
  }
  /**
   * This is the core part of the reactivity system, which makes sure that the values are updated
   * before they are read. We've also adapted it to return the loading state of the computation,
   * so that we can propagate that to the computation's observers.
   *
   * This function will ensure that the value and states we read from the computation are up to date
   */
  l() {
    if (this.a === STATE_DISPOSED) {
      throw new Error("Tried to read a disposed computation");
    }
    if (this.a === STATE_CLEAN) {
      return;
    }
    let observerFlags = 0;
    if (this.a === STATE_CHECK) {
      for (let i = 0; i < this.b.length; i++) {
        this.b[i].l();
        observerFlags |= this.b[i].i;
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
  n() {
    if (this.a === STATE_DISPOSED)
      return;
    if (this.b)
      removeSourceObservers(this, 0);
    super.n();
  }
};
function loadingState(node) {
  const prevOwner = setOwner(node.k);
  const options = void 0;
  const computation = new Computation2(
    void 0,
    () => {
      track(node);
      node.l();
      return !!(node.i & LOADING_BIT);
    },
    options
  );
  computation.p = ERROR_BIT | LOADING_BIT;
  setOwner(prevOwner);
  return computation;
}
function errorState(node) {
  const prevOwner = setOwner(node.k);
  const options = void 0;
  const computation = new Computation2(
    void 0,
    () => {
      track(node);
      node.l();
      return !!(node.i & ERROR_BIT);
    },
    options
  );
  computation.p = ERROR_BIT;
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
      updateCheck.d = computation.w > currentObserver.w;
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
    const result = compute(node, node.s, node);
    node.write(result, newFlags, true);
  } catch (error) {
    if (error instanceof NotReadyError) {
      node.write(UNCHANGED, newFlags | LOADING_BIT);
    } else {
      node.B(error);
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
        source.y?.();
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
  currentMask = observer?.p ?? DEFAULT_FLAGS;
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
    this.l();
    Computations.push(this);
  }
  m(state) {
    if (this.a >= state)
      return;
    if (this.a === STATE_CLEAN) {
      Computations.push(this);
      flushQueue();
    }
    super.m(state);
  }
};

// src/core/effect.ts
var USER_EFFECT = 0;
var RENDER_EFFECT = 1;
var Effect = class extends Computation2 {
  r;
  q = false;
  o;
  x;
  constructor(initialValue, compute2, effect, options) {
    super(initialValue, compute2, options);
    this.r = effect;
    this.o = initialValue;
    this.l();
    this.x = options?.render ? RENDER_EFFECT : USER_EFFECT;
    (this.x ? RenderEffects : Effects).push(this);
  }
  write(value) {
    if (value === UNCHANGED)
      return this.d;
    this.d = value;
    this.q = true;
    return value;
  }
  m(state) {
    if (this.a >= state)
      return;
    if (this.a === STATE_CLEAN) {
      (this.x ? RenderEffects : Effects).push(this);
      flushQueue();
    }
    this.a = state;
  }
  B(error) {
    this.handleError(error);
  }
  n() {
    this.r = void 0;
    this.o = void 0;
    super.n();
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
  let node = new Computation2(
    initialValue,
    compute2,
    options
  );
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
  void new Effect(initialValue, compute2, effect, {
    render: true,
    ...void 0
  });
}
function createRoot(init) {
  const owner = new Owner();
  return compute(
    owner,
    !init.length ? init : () => init(() => owner.dispose()),
    null
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
  owner.f = owner.f ? [handler, ...owner.f] : [handler];
  try {
    compute(owner, fn, null);
  } catch (error) {
    owner.handleError(error);
  }
}

export { Computation2 as Computation, ContextNotFoundError, NoOwnerError, NotReadyError, Owner, catchError, createAsync, createContext, createEffect, createMemo, createRenderEffect, createRoot, createSignal, flushSync, getContext, getObserver, getOwner, hasContext, hasUpdated, isEqual, isPending, latest, onCleanup, runWithOwner, setContext, untrack };
