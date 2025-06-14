// @ts-nocheck

// src/core/error.ts
var NotReadyError = class extends Error {
};
var NoOwnerError = class extends Error {
  constructor() {
    super("");
  }
};
var ContextNotFoundError = class extends Error {
  constructor() {
    super(
      ""
    );
  }
};
var EffectError = class extends Error {
  constructor(effect, cause) {
    super("");
    this.cause = cause;
  }
};

// src/core/constants.ts
var STATE_CLEAN = 0;
var STATE_CHECK = 1;
var STATE_DIRTY = 2;
var STATE_DISPOSED = 3;
var EFFECT_PURE = 0;
var EFFECT_RENDER = 1;
var EFFECT_USER = 2;
var SUPPORTS_PROXY = typeof Proxy === "function";

// src/core/scheduler.ts
var clock = 0;
function getClock() {
  return clock;
}
function incrementClock() {
  clock++;
}
var scheduled = false;
function schedule() {
  if (scheduled)
    return;
  scheduled = true;
  if (!globalQueue.J)
    queueMicrotask(flushSync);
}
var pureQueue = [];
var Queue = class {
  o = null;
  J = false;
  K = [[], []];
  E = [];
  created = clock;
  enqueue(type, node) {
    pureQueue.push(node);
    if (type)
      this.K[type - 1].push(node);
    schedule();
  }
  run(type) {
    if (type === EFFECT_PURE) {
      pureQueue.length && runPureQueue(pureQueue);
      pureQueue = [];
      return;
    } else if (this.K[type - 1].length) {
      const effects = this.K[type - 1];
      this.K[type - 1] = [];
      runEffectQueue(effects);
    }
    for (let i = 0; i < this.E.length; i++) {
      this.E[i].run(type);
    }
  }
  flush() {
    if (this.J)
      return;
    this.J = true;
    try {
      this.run(EFFECT_PURE);
      incrementClock();
      scheduled = false;
      this.run(EFFECT_RENDER);
      this.run(EFFECT_USER);
    } finally {
      this.J = false;
    }
  }
  addChild(child) {
    this.E.push(child);
    child.o = this;
  }
  removeChild(child) {
    const index = this.E.indexOf(child);
    if (index >= 0)
      this.E.splice(index, 1);
  }
  notify(...args) {
    if (this.o)
      return this.o.notify(...args);
    return false;
  }
};
var globalQueue = new Queue();
function flushSync() {
  while (scheduled) {
    globalQueue.flush();
  }
}
function runTop(node) {
  const ancestors = [];
  for (let current = node; current !== null; current = current.o) {
    if (current.a !== STATE_CLEAN) {
      ancestors.push(current);
    }
  }
  for (let i = ancestors.length - 1; i >= 0; i--) {
    if (ancestors[i].a !== STATE_DISPOSED)
      ancestors[i].x();
  }
}
function runPureQueue(queue) {
  for (let i = 0; i < queue.length; i++) {
    if (queue[i].a !== STATE_CLEAN)
      runTop(queue[i]);
  }
}
function runEffectQueue(queue) {
  for (let i = 0; i < queue.length; i++)
    queue[i].V();
}

// src/core/utils.ts
function isUndefined(value) {
  return typeof value === "undefined";
}
function tryCatch(fn) {
  try {
    const v = fn();
    if (v instanceof Promise) {
      return v.then(
        (v2) => [void 0, v2],
        (e) => {
          if (e instanceof NotReadyError)
            throw e;
          return [e];
        }
      );
    }
    return [void 0, v];
  } catch (e) {
    if (e instanceof NotReadyError)
      throw e;
    return [e];
  }
}

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
function formatId(prefix, id) {
  const num = id.toString(36), len = num.length - 1;
  return prefix + (len ? String.fromCharCode(64 + len) : "") + num;
}
var Owner = class {
  // We flatten the owner tree into a linked list so that we don't need a pointer to .firstChild
  // However, the children are actually added in reverse creation order
  // See comment at the top of the file for an example of the _nextSibling traversal
  o = null;
  m = null;
  s = null;
  a = STATE_CLEAN;
  l = null;
  p = defaultContext;
  h = globalQueue;
  W = 0;
  id = null;
  constructor(id = null, skipAppend = false) {
    this.id = id;
    if (currentOwner) {
      if (id == null && currentOwner.id != null)
        this.id = currentOwner.getNextChildId();
      !skipAppend && currentOwner.append(this);
    }
  }
  append(child) {
    child.o = this;
    child.s = this;
    if (this.m)
      this.m.s = child;
    child.m = this.m;
    this.m = child;
    if (child.p !== this.p) {
      child.p = { ...this.p, ...child.p };
    }
    if (this.h)
      child.h = this.h;
  }
  dispose(self = true) {
    if (this.a === STATE_DISPOSED)
      return;
    let head = self ? this.s || this.o : this, current = this.m, next = null;
    while (current && current.o === this) {
      current.dispose(true);
      current.y();
      next = current.m;
      current.m = null;
      current = next;
    }
    this.W = 0;
    if (self)
      this.y();
    if (current)
      current.s = !self ? this : this.s;
    if (head)
      head.m = current;
  }
  y() {
    if (this.s)
      this.s.m = null;
    this.o = null;
    this.s = null;
    this.p = defaultContext;
    this.a = STATE_DISPOSED;
    this.emptyDisposal();
  }
  emptyDisposal() {
    if (!this.l)
      return;
    if (Array.isArray(this.l)) {
      for (let i = 0; i < this.l.length; i++) {
        const callable = this.l[i];
        callable.call(callable);
      }
    } else {
      this.l.call(this.l);
    }
    this.l = null;
  }
  getNextChildId() {
    if (this.id != null)
      return formatId(this.id, this.W++);
    throw new Error("Cannot get child id from owner without an id");
  }
};
function createContext(defaultValue, description) {
  return { id: Symbol(description), defaultValue };
}
function getContext(context, owner = currentOwner) {
  if (!owner) {
    throw new NoOwnerError();
  }
  const value = hasContext(context, owner) ? owner.p[context.id] : context.defaultValue;
  if (isUndefined(value)) {
    throw new ContextNotFoundError();
  }
  return value;
}
function setContext(context, value, owner = currentOwner) {
  if (!owner) {
    throw new NoOwnerError();
  }
  owner.p = {
    ...owner.p,
    [context.id]: isUndefined(value) ? context.defaultValue : value
  };
}
function hasContext(context, owner = currentOwner) {
  return !isUndefined(owner?.p[context.id]);
}
function onCleanup(fn) {
  if (!currentOwner)
    return fn;
  const node = currentOwner;
  if (!node.l) {
    node.l = fn;
  } else if (Array.isArray(node.l)) {
    node.l.push(fn);
  } else {
    node.l = [node.l, fn];
  }
  return fn;
}

// src/core/flags.ts
var ERROR_OFFSET = 0;
var ERROR_BIT = 1 << ERROR_OFFSET;
var LOADING_OFFSET = 1;
var LOADING_BIT = 1 << LOADING_OFFSET;
var UNINITIALIZED_OFFSET = 2;
var UNINITIALIZED_BIT = 1 << UNINITIALIZED_OFFSET;
var DEFAULT_FLAGS = ERROR_BIT;

// src/core/core.ts
var currentObserver = null;
var currentMask = DEFAULT_FLAGS;
var newSources = null;
var newSourcesIndex = 0;
var newFlags = 0;
var notStale = false;
var updateCheck = null;
var staleCheck = null;
function getObserver() {
  return currentObserver;
}
var UNCHANGED = Symbol(0);
var Computation = class extends Owner {
  c = null;
  e = null;
  g;
  F;
  z;
  // Used in __DEV__ mode, hopefully removed in production
  ba;
  // Using false is an optimization as an alternative to _equals: () => false
  // which could enable more efficient DIRTY notification
  S = isEqual;
  X;
  /** Whether the computation is an error or has ancestors that are unresolved */
  f = 0;
  /** Which flags raised by sources are handled, vs. being passed through. */
  T = DEFAULT_FLAGS;
  A = -1;
  w = false;
  constructor(initialValue, compute2, options) {
    super(null, compute2 === null);
    this.z = compute2;
    this.a = compute2 ? STATE_DIRTY : STATE_CLEAN;
    this.f = compute2 && initialValue === void 0 ? UNINITIALIZED_BIT : 0;
    this.g = initialValue;
    if (options?.equals !== void 0)
      this.S = options.equals;
    if (options?.unobserved)
      this.X = options?.unobserved;
  }
  Y() {
    if (this.z) {
      if (this.f & ERROR_BIT && this.A <= getClock())
        update(this);
      else
        this.x();
    }
    track(this);
    newFlags |= this.f & ~currentMask;
    if (this.f & ERROR_BIT) {
      throw this.F;
    } else {
      return this.g;
    }
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   */
  read() {
    return this.Y();
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   *
   * If the computation has any unresolved ancestors, this function waits for the value to resolve
   * before continuing
   */
  wait() {
    if (this.z && this.f & ERROR_BIT && this.A <= getClock()) {
      update(this);
    } else {
      this.x();
    }
    track(this);
    if ((notStale || this.f & UNINITIALIZED_BIT) && this.f & LOADING_BIT) {
      throw new NotReadyError();
    }
    if (staleCheck && this.f & LOADING_BIT) {
      staleCheck.g = true;
    }
    return this.Y();
  }
  /** Update the computation with a new value. */
  write(value, flags = 0, raw = false) {
    const newValue = !raw && typeof value === "function" ? value(this.g) : value;
    const valueChanged = newValue !== UNCHANGED && (!!(this.f & UNINITIALIZED_BIT) || this.f & LOADING_BIT & ~flags || this.S === false || !this.S(this.g, newValue));
    if (valueChanged) {
      this.g = newValue;
      this.F = void 0;
    }
    const changedFlagsMask = this.f ^ flags, changedFlags = changedFlagsMask & flags;
    this.f = flags;
    this.A = getClock() + 1;
    if (this.e) {
      for (let i = 0; i < this.e.length; i++) {
        if (valueChanged) {
          this.e[i].r(STATE_DIRTY);
        } else if (changedFlagsMask) {
          this.e[i].Z(changedFlagsMask, changedFlags);
        }
      }
    }
    return this.g;
  }
  /**
   * Set the current node's state, and recursively mark all of this node's observers as STATE_CHECK
   */
  r(state, skipQueue) {
    if (this.a >= state && !this.w)
      return;
    this.w = !!skipQueue;
    this.a = state;
    if (this.e) {
      for (let i = 0; i < this.e.length; i++) {
        this.e[i].r(STATE_CHECK, skipQueue);
      }
    }
  }
  /**
   * Notify the computation that one of its sources has changed flags.
   *
   * @param mask A bitmask for which flag(s) were changed.
   * @param newFlags The source's new flags, masked to just the changed ones.
   */
  Z(mask, newFlags2) {
    if (this.a >= STATE_DIRTY)
      return;
    if (mask & this.T) {
      this.r(STATE_DIRTY);
      return;
    }
    if (this.a >= STATE_CHECK)
      return;
    const prevFlags = this.f & mask;
    const deltaFlags = prevFlags ^ newFlags2;
    if (newFlags2 === prevFlags) ; else if (deltaFlags & prevFlags & mask) {
      this.r(STATE_CHECK);
    } else {
      this.f ^= deltaFlags;
      if (this.e) {
        for (let i = 0; i < this.e.length; i++) {
          this.e[i].Z(mask, newFlags2);
        }
      }
    }
  }
  L(error) {
    this.F = error;
    this.write(UNCHANGED, this.f & ~LOADING_BIT | ERROR_BIT | UNINITIALIZED_BIT);
  }
  /**
   * This is the core part of the reactivity system, which makes sure that the values are updated
   * before they are read. We've also adapted it to return the loading state of the computation,
   * so that we can propagate that to the computation's observers.
   *
   * This function will ensure that the value and states we read from the computation are up to date
   */
  x() {
    if (!this.z) {
      return;
    }
    if (this.a === STATE_DISPOSED) {
      return;
    }
    if (this.a === STATE_CLEAN) {
      return;
    }
    let observerFlags = 0;
    if (this.a === STATE_CHECK) {
      for (let i = 0; i < this.c.length; i++) {
        this.c[i].x();
        observerFlags |= this.c[i].f;
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
  y() {
    if (this.a === STATE_DISPOSED)
      return;
    if (this.c)
      removeSourceObservers(this, 0);
    super.y();
  }
};
function track(computation) {
  if (currentObserver) {
    if (!newSources && currentObserver.c && currentObserver.c[newSourcesIndex] === computation) {
      newSourcesIndex++;
    } else if (!newSources)
      newSources = [computation];
    else if (computation !== newSources[newSources.length - 1]) {
      newSources.push(computation);
    }
    if (updateCheck) {
      updateCheck.g = computation.A > currentObserver.A;
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
      node.write(UNCHANGED, newFlags | LOADING_BIT | node.f & UNINITIALIZED_BIT);
    } else {
      node.L(error);
    }
  } finally {
    if (newSources) {
      if (node.c)
        removeSourceObservers(node, newSourcesIndex);
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
        if (!source.e)
          source.e = [node];
        else
          source.e.push(node);
      }
    } else if (node.c && newSourcesIndex < node.c.length) {
      removeSourceObservers(node, newSourcesIndex);
      node.c.length = newSourcesIndex;
    }
    newSources = prevSources;
    newSourcesIndex = prevSourcesIndex;
    newFlags = prevFlags;
    node.A = getClock() + 1;
    node.a = STATE_CLEAN;
  }
}
function removeSourceObservers(node, index) {
  let source;
  let swap;
  for (let i = index; i < node.c.length; i++) {
    source = node.c[i];
    if (source.e) {
      swap = source.e.indexOf(node);
      source.e[swap] = source.e[source.e.length - 1];
      source.e.pop();
      if (!source.e.length)
        source.X?.();
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
  updateCheck = { g: false };
  try {
    fn();
    return updateCheck.g;
  } finally {
    updateCheck = current;
  }
}
function pendingCheck(fn, loadingValue) {
  const current = staleCheck;
  staleCheck = { g: false };
  try {
    latest(fn);
    return staleCheck.g;
  } catch (err) {
    if (!(err instanceof NotReadyError))
      return false;
    if (loadingValue !== void 0)
      return loadingValue;
    throw err;
  } finally {
    staleCheck = current;
  }
}
function isPending(fn, loadingValue) {
  if (!currentObserver)
    return pendingCheck(fn, loadingValue);
  const c = new Computation(void 0, () => pendingCheck(fn, loadingValue));
  c.T |= LOADING_BIT;
  return c.read();
}
function latest(fn, fallback) {
  const argLength = arguments.length;
  const prevFlags = newFlags;
  const prevNotStale = notStale;
  notStale = false;
  try {
    return fn();
  } catch (err) {
    if (argLength > 1 && err instanceof NotReadyError)
      return fallback;
    throw err;
  } finally {
    newFlags = prevFlags;
    notStale = prevNotStale;
  }
}
function runWithObserver(observer, run) {
  const prevSources = newSources, prevSourcesIndex = newSourcesIndex, prevFlags = newFlags;
  newSources = null;
  newSourcesIndex = observer.c ? observer.c.length : 0;
  newFlags = 0;
  try {
    return compute(observer, run, observer);
  } catch (error) {
    if (error instanceof NotReadyError) {
      observer.write(
        UNCHANGED,
        newFlags | LOADING_BIT | observer.f & UNINITIALIZED_BIT
      );
    } else {
      observer.L(error);
    }
  } finally {
    if (newSources) {
      if (newSourcesIndex > 0) {
        observer.c.length = newSourcesIndex + newSources.length;
        for (let i = 0; i < newSources.length; i++) {
          observer.c[newSourcesIndex + i] = newSources[i];
        }
      } else {
        observer.c = newSources;
      }
      let source;
      for (let i = newSourcesIndex; i < observer.c.length; i++) {
        source = observer.c[i];
        if (!source.e)
          source.e = [observer];
        else
          source.e.push(observer);
      }
    }
    newSources = prevSources;
    newSourcesIndex = prevSourcesIndex;
    newFlags = prevFlags;
  }
}
function compute(owner, fn, observer) {
  const prevOwner = setOwner(owner), prevObserver = currentObserver, prevMask = currentMask, prevNotStale = notStale;
  currentObserver = observer;
  currentMask = observer?.T ?? DEFAULT_FLAGS;
  notStale = true;
  try {
    return fn(observer ? observer.g : void 0);
  } finally {
    setOwner(prevOwner);
    currentObserver = prevObserver;
    currentMask = prevMask;
    notStale = prevNotStale;
  }
}
function flatten(children, options) {
  try {
    if (typeof children === "function" && !children.length) {
      if (options?.doNotUnwrap)
        return children;
      do {
        children = children();
      } while (typeof children === "function" && !children.length);
    }
    if (options?.skipNonRendered && (children == null || children === true || children === false || children === ""))
      return;
    if (Array.isArray(children)) {
      let results = [];
      if (flattenArray(children, results, options)) {
        return () => {
          let nested = [];
          flattenArray(results, nested, { ...options, doNotUnwrap: false });
          return nested;
        };
      }
      return results;
    }
    return children;
  } catch (e) {
    if (options?.skipNonRendered && e instanceof NotReadyError) {
      newFlags |= LOADING_BIT;
      return void 0;
    }
    throw e;
  }
}
function flattenArray(children, results = [], options) {
  let notReady = null;
  let needsUnwrap = false;
  for (let i = 0; i < children.length; i++) {
    try {
      let child = children[i];
      if (typeof child === "function" && !child.length) {
        if (options?.doNotUnwrap) {
          results.push(child);
          needsUnwrap = true;
          continue;
        }
        do {
          child = child();
        } while (typeof child === "function" && !child.length);
      }
      if (Array.isArray(child)) {
        needsUnwrap = flattenArray(child, results, options);
      } else if (options?.skipNonRendered && (child == null || child === true || child === false || child === "")) {
      } else
        results.push(child);
    } catch (e) {
      if (!(e instanceof NotReadyError))
        throw e;
      notReady = e;
    }
  }
  if (notReady)
    throw notReady;
  return needsUnwrap;
}

// src/core/effect.ts
var Effect = class extends Computation {
  M;
  N;
  B;
  U = false;
  O;
  t;
  constructor(initialValue, compute2, effect, error, options) {
    super(initialValue, compute2, options);
    this.M = effect;
    this.N = error;
    this.O = initialValue;
    this.t = options?.render ? EFFECT_RENDER : EFFECT_USER;
    if (this.t === EFFECT_RENDER) {
      this.z = (p) => getClock() > this.h.created && !(this.f & ERROR_BIT) ? latest(() => compute2(p)) : compute2(p);
    }
    this.x();
    !options?.defer && (this.t === EFFECT_USER ? this.h.enqueue(this.t, this) : this.V());
  }
  write(value, flags = 0) {
    if (this.a == STATE_DIRTY) {
      this.f;
      this.f = flags;
      if (this.t === EFFECT_RENDER) {
        this.h.notify(this, LOADING_BIT | ERROR_BIT, flags);
      }
    }
    if (value === UNCHANGED)
      return this.g;
    this.g = value;
    this.U = true;
    return value;
  }
  r(state, skipQueue) {
    if (this.a >= state || skipQueue)
      return;
    if (this.a === STATE_CLEAN)
      this.h.enqueue(this.t, this);
    this.a = state;
  }
  L(error) {
    this.F = error;
    this.B?.();
    this.h.notify(this, LOADING_BIT, 0);
    this.f = ERROR_BIT;
    if (this.t === EFFECT_USER) {
      try {
        return this.N ? this.B = this.N(error) : console.error(new EffectError(this.M, error));
      } catch (e) {
        error = e;
      }
    }
    if (!this.h.notify(this, ERROR_BIT, ERROR_BIT))
      throw error;
  }
  y() {
    if (this.a === STATE_DISPOSED)
      return;
    this.M = void 0;
    this.O = void 0;
    this.N = void 0;
    this.B?.();
    this.B = void 0;
    super.y();
  }
  V() {
    if (this.U && this.a !== STATE_DISPOSED) {
      this.B?.();
      try {
        this.B = this.M(this.g, this.O);
      } catch (e) {
        if (!this.h.notify(this, ERROR_BIT, ERROR_BIT))
          throw e;
      } finally {
        this.O = this.g;
        this.U = false;
      }
    }
  }
};
var EagerComputation = class extends Computation {
  constructor(initialValue, compute2, options) {
    super(initialValue, compute2, options);
    !options?.defer && this.x();
  }
  r(state, skipQueue) {
    if (this.a >= state && !this.w)
      return;
    if (!skipQueue && (this.a === STATE_CLEAN || this.a === STATE_CHECK && this.w))
      this.h.enqueue(EFFECT_PURE, this);
    super.r(state, skipQueue);
  }
};
var ProjectionComputation = class extends Computation {
  constructor(compute2) {
    super(void 0, compute2);
  }
  r(state, skipQueue) {
    if (this.a >= state && !this.w)
      return;
    if (!skipQueue && (this.a === STATE_CLEAN || this.a === STATE_CHECK && this.w))
      this.h.enqueue(EFFECT_PURE, this);
    super.r(state, true);
    this.w = !!skipQueue;
  }
};

// src/core/boundaries.ts
var BoundaryComputation = class extends EagerComputation {
  G;
  constructor(compute2, propagationMask) {
    super(void 0, compute2, { defer: true });
    this.G = propagationMask;
  }
  write(value, flags) {
    super.write(value, flags & ~this.G);
    if (this.G & LOADING_BIT && !(this.f & UNINITIALIZED_BIT)) {
      flags &= ~LOADING_BIT;
    }
    this.h.notify(this, this.G, flags);
    return this.g;
  }
};
function createBoundChildren(owner, fn, queue, mask) {
  const parentQueue = owner.h;
  parentQueue.addChild(owner.h = queue);
  onCleanup(() => parentQueue.removeChild(owner.h));
  return compute(
    owner,
    () => {
      const c = new Computation(void 0, fn);
      return new BoundaryComputation(() => flatten(c.wait()), mask);
    },
    null
  );
}
var ConditionalQueue = class extends Queue {
  n;
  P = /* @__PURE__ */ new Set();
  Q = /* @__PURE__ */ new Set();
  constructor(disabled) {
    super();
    this.n = disabled;
  }
  run(type) {
    if (!type || this.n.read())
      return;
    return super.run(type);
  }
  notify(node, type, flags) {
    if (this.n.read()) {
      if (type === LOADING_BIT) {
        flags & LOADING_BIT ? this.Q.add(node) : this.Q.delete(node);
      }
      if (type === ERROR_BIT) {
        flags & ERROR_BIT ? this.P.add(node) : this.P.delete(node);
      }
      return true;
    }
    return super.notify(node, type, flags);
  }
};
var CollectionQueue = class extends Queue {
  R;
  b = /* @__PURE__ */ new Set();
  n = new Computation(false, null);
  constructor(type) {
    super();
    this.R = type;
  }
  run(type) {
    if (!type || this.n.read())
      return;
    return super.run(type);
  }
  notify(node, type, flags) {
    if (!(type & this.R))
      return super.notify(node, type, flags);
    if (flags & this.R) {
      this.b.add(node);
      if (this.b.size === 1)
        this.n.write(true);
    } else {
      this.b.delete(node);
      if (this.b.size === 0)
        this.n.write(false);
    }
    type &= ~this.R;
    return type ? super.notify(node, type, flags) : true;
  }
};
function createBoundary(fn, condition) {
  const owner = new Owner();
  const queue = new ConditionalQueue(new Computation(void 0, () => condition() === "hidden" /* HIDDEN */));
  const tree = createBoundChildren(owner, fn, queue, 0);
  new EagerComputation(void 0, () => {
    const disabled = queue.n.read();
    tree.G = disabled ? ERROR_BIT | LOADING_BIT : 0;
    if (!disabled) {
      queue.Q.forEach((node) => queue.notify(node, LOADING_BIT, LOADING_BIT));
      queue.P.forEach((node) => queue.notify(node, ERROR_BIT, ERROR_BIT));
      queue.Q.clear();
      queue.P.clear();
    }
  });
  return () => queue.n.read() ? void 0 : tree.read();
}
function createCollectionBoundary(type, fn, fallback) {
  const owner = new Owner();
  const queue = new CollectionQueue(type);
  const tree = createBoundChildren(owner, fn, queue, type);
  const decision = new Computation(void 0, () => {
    if (!queue.n.read()) {
      const resolved = tree.read();
      if (!queue.n.read())
        return resolved;
    }
    return fallback(queue);
  });
  return decision.read.bind(decision);
}
function createSuspense(fn, fallback) {
  return createCollectionBoundary(LOADING_BIT, fn, () => fallback());
}
function createErrorBoundary(fn, fallback) {
  return createCollectionBoundary(
    ERROR_BIT,
    fn,
    (queue) => fallback(queue.b.values().next().value.F, () => {
      incrementClock();
      for (let node of queue.b) {
        node.a = STATE_DIRTY;
        node.h?.enqueue(node.t, node);
      }
    })
  );
}

// src/signals.ts
function createSignal(first, second, third) {
  if (typeof first === "function") {
    const memo = createMemo((p) => {
      const node2 = new Computation(
        first(p ? untrack(p[0]) : second),
        null,
        third
      );
      return [node2.read.bind(node2), node2.write.bind(node2)];
    });
    return [() => memo()[0](), (value) => memo()[1](value)];
  }
  const node = new Computation(first, null, second);
  return [node.read.bind(node), node.write.bind(node)];
}
function createMemo(compute2, value, options) {
  let node = new Computation(
    value,
    compute2,
    options
  );
  let resolvedValue;
  return () => {
    if (node) {
      if (node.a === STATE_DISPOSED) {
        node = void 0;
        return resolvedValue;
      }
      resolvedValue = node.wait();
      if (!node.c?.length && node.m?.o !== node) {
        node.dispose();
        node = void 0;
      }
    }
    return resolvedValue;
  };
}
function createAsync(compute2, value, options) {
  const node = new EagerComputation(
    value,
    (p) => {
      const source = compute2(p);
      const isPromise = source instanceof Promise;
      const iterator = source[Symbol.asyncIterator];
      if (!isPromise && !iterator) {
        return source;
      }
      let abort = false;
      onCleanup(() => abort = true);
      if (isPromise) {
        source.then(
          (value3) => {
            if (abort)
              return;
            node.write(value3, 0, true);
          },
          (error) => {
            if (abort)
              return;
            node.L(error);
          }
        );
      } else {
        (async () => {
          try {
            for await (let value3 of source) {
              if (abort)
                return;
              node.write(value3, 0, true);
            }
          } catch (error) {
            if (abort)
              return;
            node.write(error, ERROR_BIT);
          }
        })();
      }
      throw new NotReadyError();
    },
    options
  );
  return node.wait.bind(node);
}
function createEffect(compute2, effect, error, value, options) {
  void new Effect(
    value,
    compute2,
    effect,
    error,
    options
  );
}
function createRenderEffect(compute2, effect, value, options) {
  void new Effect(value, compute2, effect, void 0, {
    render: true,
    ...options
  });
}
function createRoot(init, options) {
  const owner = new Owner(options?.id);
  return compute(owner, !init.length ? init : () => init(() => owner.dispose()), null);
}
function runWithOwner(owner, run) {
  return compute(owner, run, null);
}
function resolve(fn) {
  return new Promise((res, rej) => {
    createRoot((dispose) => {
      new EagerComputation(void 0, () => {
        try {
          res(fn());
        } catch (err) {
          if (err instanceof NotReadyError)
            throw err;
          rej(err);
        }
        dispose();
      });
    });
  });
}

// src/store/projection.ts
function createProjection(fn, initialValue = {}) {
  const [store] = createStore(fn, initialValue);
  return store;
}
function wrapProjection(fn, store, setStore) {
  const node = new ProjectionComputation(() => {
    setStore(fn);
  });
  const wrapped = /* @__PURE__ */ new WeakMap();
  return [wrap(store, node, wrapped), setStore];
}
function wrap(source, node, wrapped) {
  if (wrapped.has(source))
    return wrapped.get(source);
  const wrap3 = new Proxy(source, {
    get(target, property) {
      node.read();
      const v = target[property];
      return isWrappable(v) ? wrap3(v, node, wrapped) : v;
    },
    set() {
      throw new Error("Projections are readonly");
    },
    deleteProperty() {
      throw new Error("Projections are readonly");
    }
  });
  wrapped.set(source, wrap3);
  return wrap3;
}

// src/store/store.ts
var $RAW = Symbol(0);
var $TRACK = Symbol(0);
var $DEEP = Symbol(0);
var $TARGET = Symbol(0);
var $PROXY = Symbol(0);
var PARENTS = /* @__PURE__ */ new WeakMap();
var STORE_VALUE = "v";
var STORE_NODE = "n";
var STORE_HAS = "h";
function wrap2(value) {
  let p = value[$PROXY];
  if (!p) {
    let target;
    if (Array.isArray(value)) {
      target = [];
      target.v = value;
    } else
      target = { v: value };
    Object.defineProperty(value, $PROXY, {
      value: p = new Proxy(target, proxyTraps),
      writable: true
    });
  }
  return p;
}
function isWrappable(obj) {
  return obj != null && typeof obj === "object" && !Object.isFrozen(obj);
}
function unwrap(item, deep2 = true, set) {
  let result, unwrapped, v, prop;
  if (result = item != null && item[$RAW])
    return result;
  if (!deep2)
    return item;
  if (!isWrappable(item) || set?.has(item))
    return item;
  if (!set)
    set = /* @__PURE__ */ new Set();
  set.add(item);
  if (Array.isArray(item)) {
    for (let i = 0, l = item.length; i < l; i++) {
      v = item[i];
      if ((unwrapped = unwrap(v, deep2, set)) !== v)
        item[i] = unwrapped;
    }
  } else {
    if (!deep2)
      return item;
    const keys = Object.keys(item);
    for (let i = 0, l = keys.length; i < l; i++) {
      prop = keys[i];
      const desc = Object.getOwnPropertyDescriptor(item, prop);
      if (desc.get)
        continue;
      v = item[prop];
      if ((unwrapped = unwrap(v, deep2, set)) !== v)
        item[prop] = unwrapped;
    }
  }
  return item;
}
function getNodes(target, type) {
  let nodes = target[type];
  if (!nodes)
    target[type] = nodes = /* @__PURE__ */ Object.create(null);
  return nodes;
}
function getNode(nodes, property, value, equals = isEqual) {
  if (nodes[property])
    return nodes[property];
  return nodes[property] = new Computation(value, null, {
    equals,
    unobserved() {
      delete nodes[property];
    }
  });
}
function proxyDescriptor(target, property) {
  if (property === $PROXY)
    return { value: target[$PROXY], writable: true, configurable: true };
  const desc = Reflect.getOwnPropertyDescriptor(target[STORE_VALUE], property);
  if (!desc || desc.get || !desc.configurable)
    return desc;
  delete desc.value;
  delete desc.writable;
  desc.get = () => target[STORE_VALUE][$PROXY][property];
  return desc;
}
function trackSelf(target, symbol = $TRACK) {
  getObserver() && getNode(getNodes(target, STORE_NODE), symbol, void 0, false).read();
}
function ownKeys(target) {
  trackSelf(target);
  return Reflect.ownKeys(target[STORE_VALUE]);
}
var Writing = null;
var proxyTraps = {
  get(target, property, receiver) {
    if (property === $TARGET)
      return target;
    if (property === $RAW)
      return target[STORE_VALUE];
    if (property === $PROXY)
      return receiver;
    if (property === $TRACK || property === $DEEP) {
      trackSelf(target, property);
      return receiver;
    }
    const nodes = getNodes(target, STORE_NODE);
    const storeValue = target[STORE_VALUE];
    const tracked = nodes[property];
    if (!tracked) {
      const desc = Object.getOwnPropertyDescriptor(storeValue, property);
      if (desc && desc.get)
        return desc.get.call(receiver);
    }
    if (Writing?.has(storeValue)) {
      const value2 = tracked ? tracked.g : storeValue[property];
      return isWrappable(value2) ? (Writing.add(value2[$RAW] || value2), wrap2(value2)) : value2;
    }
    let value = tracked ? nodes[property].read() : storeValue[property];
    if (!tracked) {
      if (typeof value === "function" && !storeValue.hasOwnProperty(property)) {
        let proto;
        return !Array.isArray(storeValue) && (proto = Object.getPrototypeOf(storeValue)) && proto !== Object.prototype ? value.bind(storeValue) : value;
      } else if (getObserver()) {
        return getNode(nodes, property, isWrappable(value) ? wrap2(value) : value).read();
      }
    }
    return isWrappable(value) ? wrap2(value) : value;
  },
  has(target, property) {
    if (property === $RAW || property === $PROXY || property === $TRACK || property === "__proto__")
      return true;
    const has = property in target[STORE_VALUE];
    getObserver() && getNode(getNodes(target, STORE_HAS), property, has).read();
    return has;
  },
  set(target, property, value) {
    Writing?.has(target[STORE_VALUE]) && setProperty(target[STORE_VALUE], property, unwrap(value, false));
    return true;
  },
  deleteProperty(target, property) {
    Writing?.has(target[STORE_VALUE]) && setProperty(target[STORE_VALUE], property, void 0, true);
    return true;
  },
  ownKeys,
  getOwnPropertyDescriptor: proxyDescriptor,
  getPrototypeOf(target) {
    return Object.getPrototypeOf(target[STORE_VALUE]);
  }
};
function setProperty(state, property, value, deleting = false) {
  const prev = state[property];
  if (!deleting && prev === value)
    return;
  const len = state.length;
  if (deleting)
    delete state[property];
  else
    state[property] = value;
  const wrappable = isWrappable(value);
  if (isWrappable(prev)) {
    const parents = PARENTS.get(prev);
    parents && (parents instanceof Set ? parents.delete(state) : PARENTS.delete(prev));
  }
  if (recursivelyNotify(state) && wrappable)
    recursivelyAddParent(value[$RAW] || value, state);
  const target = state[$PROXY]?.[$TARGET];
  if (!target)
    return;
  if (deleting)
    target[STORE_HAS]?.[property]?.write(false);
  else
    target[STORE_HAS]?.[property]?.write(true);
  const nodes = getNodes(target, STORE_NODE);
  nodes[property]?.write(wrappable ? wrap2(value) : value);
  Array.isArray(state) && state.length !== len && nodes.length?.write(state.length);
  nodes[$TRACK]?.write(void 0);
}
function recursivelyNotify(state) {
  let target = state[$PROXY]?.[$TARGET];
  let notified = false;
  target && (getNodes(target, STORE_NODE)[$DEEP]?.write(void 0), notified = true);
  const parents = PARENTS.get(state);
  if (!parents)
    return notified;
  if (parents instanceof Set) {
    for (let parent of parents)
      notified = recursivelyNotify(parent) || notified;
  } else
    notified = recursivelyNotify(parents) || notified;
  return notified;
}
function recursivelyAddParent(state, parent) {
  if (parent) {
    let parents = PARENTS.get(state);
    if (!parents)
      PARENTS.set(state, parent);
    else if (parents !== parent) {
      if (!(parents instanceof Set))
        PARENTS.set(state, parents = /* @__PURE__ */ new Set([parents]));
      else if (parents.has(parent))
        return;
      parents.add(parent);
    } else
      return;
  }
  if (Array.isArray(state)) {
    for (let i = 0; i < state.length; i++) {
      const item = state[i];
      isWrappable(item) && recursivelyAddParent(item[$RAW] || item, state);
    }
  } else {
    const keys = Object.keys(state);
    for (let i = 0; i < keys.length; i++) {
      const item = state[keys[i]];
      isWrappable(item) && recursivelyAddParent(item[$RAW] || item, state);
    }
  }
}
function createStore(first, second) {
  const derived = typeof first === "function", store = derived ? second : first;
  const unwrappedStore = unwrap(store);
  let wrappedStore = wrap2(unwrappedStore);
  const setStore = (fn) => {
    const prevWriting = Writing;
    Writing = /* @__PURE__ */ new Set();
    Writing.add(unwrappedStore);
    try {
      fn(wrappedStore);
    } finally {
      Writing.clear();
      Writing = prevWriting;
    }
  };
  if (derived)
    return wrapProjection(first, wrappedStore, setStore);
  return [wrappedStore, setStore];
}
function deep(store) {
  recursivelyAddParent(store[$RAW] || store);
  return store[$DEEP];
}

// src/store/reconcile.ts
function applyState(next, state, keyFn) {
  const target = state?.[$TARGET];
  if (!target)
    return;
  const previous = target[STORE_VALUE];
  if (next === previous)
    return;
  Object.defineProperty(next, $PROXY, {
    value: previous[$PROXY],
    writable: true
  });
  previous[$PROXY] = null;
  target[STORE_VALUE] = next;
  if (Array.isArray(previous)) {
    let changed = false;
    if (next.length && previous.length && next[0] && keyFn(next[0]) != null) {
      let i, j, start, end, newEnd, item, newIndicesNext, keyVal;
      for (start = 0, end = Math.min(previous.length, next.length); start < end && (previous[start] === next[start] || previous[start] && next[start] && keyFn(previous[start]) === keyFn(next[start])); start++) {
        applyState(next[start], wrap2(previous[start]), keyFn);
      }
      const temp = new Array(next.length), newIndices = /* @__PURE__ */ new Map();
      for (end = previous.length - 1, newEnd = next.length - 1; end >= start && newEnd >= start && (previous[end] === next[newEnd] || previous[end] && next[newEnd] && keyFn(previous[end]) === keyFn(next[newEnd])); end--, newEnd--) {
        temp[newEnd] = previous[end];
      }
      if (start > newEnd || start > end) {
        for (j = start; j <= newEnd; j++) {
          changed = true;
          target[STORE_NODE][j]?.write(wrap2(next[j]));
        }
        for (; j < next.length; j++) {
          changed = true;
          const wrapped = wrap2(temp[j]);
          target[STORE_NODE][j]?.write(wrapped);
          applyState(next[j], wrapped, keyFn);
        }
        changed && target[STORE_NODE][$TRACK]?.write(void 0);
        previous.length !== next.length && target[STORE_NODE].length?.write(next.length);
        return;
      }
      newIndicesNext = new Array(newEnd + 1);
      for (j = newEnd; j >= start; j--) {
        item = next[j];
        keyVal = item ? keyFn(item) : item;
        i = newIndices.get(keyVal);
        newIndicesNext[j] = i === void 0 ? -1 : i;
        newIndices.set(keyVal, j);
      }
      for (i = start; i <= end; i++) {
        item = previous[i];
        keyVal = item ? keyFn(item) : item;
        j = newIndices.get(keyVal);
        if (j !== void 0 && j !== -1) {
          temp[j] = previous[i];
          j = newIndicesNext[j];
          newIndices.set(keyVal, j);
        }
      }
      for (j = start; j < next.length; j++) {
        if (j in temp) {
          const wrapped = wrap2(temp[j]);
          target[STORE_NODE][j]?.write(wrapped);
          applyState(next[j], wrapped, keyFn);
        } else
          target[STORE_NODE][j]?.write(wrap2(next[j]));
      }
      if (start < next.length)
        changed = true;
    } else if (previous.length && next.length) {
      for (let i = 0, len = next.length; i < len; i++) {
        isWrappable(previous[i]) && applyState(next[i], wrap2(previous[i]), keyFn);
      }
    }
    if (previous.length !== next.length) {
      changed = true;
      target[STORE_NODE].length?.write(next.length);
    }
    changed && target[STORE_NODE][$TRACK]?.write(void 0);
    return;
  }
  let nodes = target[STORE_NODE];
  if (nodes) {
    const keys = Object.keys(nodes);
    for (let i = 0, len = keys.length; i < len; i++) {
      const node = nodes[keys[i]];
      const previousValue = unwrap(previous[keys[i]], false);
      let nextValue = unwrap(next[keys[i]], false);
      if (previousValue === nextValue)
        continue;
      if (!previousValue || !isWrappable(previousValue) || keyFn(previousValue) != null && keyFn(previousValue) !== keyFn(nextValue))
        node.write(isWrappable(nextValue) ? wrap2(nextValue) : nextValue);
      else
        applyState(nextValue, wrap2(previousValue), keyFn);
    }
  }
  if (nodes = target[STORE_HAS]) {
    const keys = Object.keys(nodes);
    for (let i = 0, len = keys.length; i < len; i++) {
      nodes[keys[i]].write(keys[i] in next);
    }
  }
}
function reconcile(value, key) {
  return (state) => {
    const keyFn = typeof key === "string" ? (item) => item[key] : key;
    if (keyFn(value) !== keyFn(state))
      throw new Error("Cannot reconcile states with different identity");
    applyState(value, state, keyFn);
    return state;
  };
}

// src/store/utils.ts
function trueFn() {
  return true;
}
var propTraps = {
  get(_, property, receiver) {
    if (property === $PROXY)
      return receiver;
    return _.get(property);
  },
  has(_, property) {
    if (property === $PROXY)
      return true;
    return _.has(property);
  },
  set: trueFn,
  deleteProperty: trueFn,
  getOwnPropertyDescriptor(_, property) {
    return {
      configurable: true,
      enumerable: true,
      get() {
        return _.get(property);
      },
      set: trueFn,
      deleteProperty: trueFn
    };
  },
  ownKeys(_) {
    return _.keys();
  }
};
function resolveSource(s) {
  return !(s = typeof s === "function" ? s() : s) ? {} : s;
}
var $SOURCES = Symbol(0);
function merge(...sources) {
  if (sources.length === 1 && typeof sources[0] !== "function")
    return sources[0];
  let proxy = false;
  const flattened = [];
  for (let i = 0; i < sources.length; i++) {
    const s = sources[i];
    proxy = proxy || !!s && $PROXY in s;
    const childSources = !!s && s[$SOURCES];
    if (childSources)
      flattened.push(...childSources);
    else
      flattened.push(
        typeof s === "function" ? (proxy = true, createMemo(s)) : s
      );
  }
  if (SUPPORTS_PROXY && proxy) {
    return new Proxy(
      {
        get(property) {
          if (property === $SOURCES)
            return flattened;
          for (let i = flattened.length - 1; i >= 0; i--) {
            const s = resolveSource(flattened[i]);
            if (property in s)
              return s[property];
          }
        },
        has(property) {
          for (let i = flattened.length - 1; i >= 0; i--) {
            if (property in resolveSource(flattened[i]))
              return true;
          }
          return false;
        },
        keys() {
          const keys = [];
          for (let i = 0; i < flattened.length; i++)
            keys.push(...Object.keys(resolveSource(flattened[i])));
          return [...new Set(keys)];
        }
      },
      propTraps
    );
  }
  const defined = /* @__PURE__ */ Object.create(null);
  let nonTargetKey = false;
  let lastIndex = flattened.length - 1;
  for (let i = lastIndex; i >= 0; i--) {
    const source = flattened[i];
    if (!source) {
      i === lastIndex && lastIndex--;
      continue;
    }
    const sourceKeys = Object.getOwnPropertyNames(source);
    for (let j = sourceKeys.length - 1; j >= 0; j--) {
      const key = sourceKeys[j];
      if (key === "__proto__" || key === "constructor")
        continue;
      if (!defined[key]) {
        nonTargetKey = nonTargetKey || i !== lastIndex;
        const desc = Object.getOwnPropertyDescriptor(source, key);
        defined[key] = desc.get ? {
          enumerable: true,
          configurable: true,
          get: desc.get.bind(source)
        } : desc;
      }
    }
  }
  if (!nonTargetKey)
    return flattened[lastIndex];
  const target = {};
  const definedKeys = Object.keys(defined);
  for (let i = definedKeys.length - 1; i >= 0; i--) {
    const key = definedKeys[i], desc = defined[key];
    if (desc.get)
      Object.defineProperty(target, key, desc);
    else
      target[key] = desc.value;
  }
  target[$SOURCES] = flattened;
  return target;
}
function omit(props, ...keys) {
  const blocked = new Set(keys);
  if (SUPPORTS_PROXY && $PROXY in props) {
    return new Proxy(
      {
        get(property) {
          return blocked.has(property) ? void 0 : props[property];
        },
        has(property) {
          return !blocked.has(property) && property in props;
        },
        keys() {
          return Object.keys(props).filter((k) => !blocked.has(k));
        }
      },
      propTraps
    );
  }
  const result = {};
  for (const propName of Object.getOwnPropertyNames(props)) {
    if (!blocked.has(propName)) {
      const desc = Object.getOwnPropertyDescriptor(props, propName);
      !desc.get && !desc.set && desc.enumerable && desc.writable && desc.configurable ? result[propName] = desc.value : Object.defineProperty(result, propName, desc);
    }
  }
  return result;
}

// src/map.ts
function mapArray(list, map, options) {
  const keyFn = typeof options?.keyed === "function" ? options.keyed : void 0;
  return updateKeyedMap.bind({
    H: new Owner(),
    i: 0,
    _: list,
    u: [],
    C: map,
    d: [],
    b: [],
    D: keyFn,
    j: keyFn || options?.keyed === false ? [] : void 0,
    k: map.length > 1 ? [] : void 0,
    I: options?.fallback
  });
}
function updateKeyedMap() {
  const newItems = this._() || [], newLen = newItems.length;
  newItems[$TRACK];
  runWithOwner(this.H, () => {
    let i, j, mapper = this.j ? () => {
      this.j[j] = new Computation(newItems[j], null);
      this.k && (this.k[j] = new Computation(j, null));
      return this.C(
        Computation.prototype.read.bind(this.j[j]),
        this.k ? Computation.prototype.read.bind(this.k[j]) : void 0
      );
    } : this.k ? () => {
      const item = newItems[j];
      this.k[j] = new Computation(j, null);
      return this.C(() => item, Computation.prototype.read.bind(this.k[j]));
    } : () => {
      const item = newItems[j];
      return this.C(() => item);
    };
    if (newLen === 0) {
      if (this.i !== 0) {
        this.H.dispose(false);
        this.b = [];
        this.u = [];
        this.d = [];
        this.i = 0;
        this.j && (this.j = []);
        this.k && (this.k = []);
      }
      if (this.I && !this.d[0]) {
        this.d[0] = compute(
          this.b[0] = new Owner(),
          this.I,
          null
        );
      }
    } else if (this.i === 0) {
      if (this.b[0])
        this.b[0].dispose();
      this.d = new Array(newLen);
      for (j = 0; j < newLen; j++) {
        this.u[j] = newItems[j];
        this.d[j] = compute(this.b[j] = new Owner(), mapper, null);
      }
      this.i = newLen;
    } else {
      let start, end, newEnd, item, key, newIndices, newIndicesNext, temp = new Array(newLen), tempNodes = new Array(newLen), tempRows = this.j ? new Array(newLen) : void 0, tempIndexes = this.k ? new Array(newLen) : void 0;
      for (start = 0, end = Math.min(this.i, newLen); start < end && (this.u[start] === newItems[start] || this.j && compare(this.D, this.u[start], newItems[start])); start++) {
        if (this.j)
          this.j[start].write(newItems[start]);
      }
      for (end = this.i - 1, newEnd = newLen - 1; end >= start && newEnd >= start && (this.u[end] === newItems[newEnd] || this.j && compare(this.D, this.u[end], newItems[newEnd])); end--, newEnd--) {
        temp[newEnd] = this.d[end];
        tempNodes[newEnd] = this.b[end];
        tempRows && (tempRows[newEnd] = this.j[end]);
        tempIndexes && (tempIndexes[newEnd] = this.k[end]);
      }
      newIndices = /* @__PURE__ */ new Map();
      newIndicesNext = new Array(newEnd + 1);
      for (j = newEnd; j >= start; j--) {
        item = newItems[j];
        key = this.D ? this.D(item) : item;
        i = newIndices.get(key);
        newIndicesNext[j] = i === void 0 ? -1 : i;
        newIndices.set(key, j);
      }
      for (i = start; i <= end; i++) {
        item = this.u[i];
        key = this.D ? this.D(item) : item;
        j = newIndices.get(key);
        if (j !== void 0 && j !== -1) {
          temp[j] = this.d[i];
          tempNodes[j] = this.b[i];
          tempRows && (tempRows[j] = this.j[i]);
          tempIndexes && (tempIndexes[j] = this.k[i]);
          j = newIndicesNext[j];
          newIndices.set(key, j);
        } else
          this.b[i].dispose();
      }
      for (j = start; j < newLen; j++) {
        if (j in temp) {
          this.d[j] = temp[j];
          this.b[j] = tempNodes[j];
          if (tempRows) {
            this.j[j] = tempRows[j];
            this.j[j].write(newItems[j]);
          }
          if (tempIndexes) {
            this.k[j] = tempIndexes[j];
            this.k[j].write(j);
          }
        } else {
          this.d[j] = compute(this.b[j] = new Owner(), mapper, null);
        }
      }
      this.d = this.d.slice(0, this.i = newLen);
      this.u = newItems.slice(0);
    }
  });
  return this.d;
}
function repeat(count, map, options) {
  return updateRepeat.bind({
    H: new Owner(),
    i: 0,
    q: 0,
    $: count,
    C: map,
    b: [],
    d: [],
    aa: options?.from,
    I: options?.fallback
  });
}
function updateRepeat() {
  const newLen = this.$();
  const from = this.aa?.() || 0;
  runWithOwner(this.H, () => {
    if (newLen === 0) {
      if (this.i !== 0) {
        this.H.dispose(false);
        this.b = [];
        this.d = [];
        this.i = 0;
      }
      if (this.I && !this.d[0]) {
        this.d[0] = compute(
          this.b[0] = new Owner(),
          this.I,
          null
        );
      }
      return;
    }
    const to = from + newLen;
    const prevTo = this.q + this.i;
    if (this.i === 0 && this.b[0])
      this.b[0].dispose();
    for (let i = to; i < prevTo; i++)
      this.b[i - this.q].dispose();
    if (this.q < from) {
      let i = this.q;
      while (i < from && i < this.i)
        this.b[i++].dispose();
      this.b.splice(0, from - this.q);
      this.d.splice(0, from - this.q);
    } else if (this.q > from) {
      let i = prevTo - this.q - 1;
      let difference = this.q - from;
      this.b.length = this.d.length = newLen;
      while (i >= difference) {
        this.b[i] = this.b[i - difference];
        this.d[i] = this.d[i - difference];
        i--;
      }
      for (let i2 = 0; i2 < difference; i2++) {
        this.d[i2] = compute(
          this.b[i2] = new Owner(),
          () => this.C(i2 + from),
          null
        );
      }
    }
    for (let i = prevTo; i < to; i++) {
      this.d[i - from] = compute(
        this.b[i - from] = new Owner(),
        () => this.C(i),
        null
      );
    }
    this.d = this.d.slice(0, newLen);
    this.q = from;
    this.i = newLen;
  });
  return this.d;
}
function compare(key, a, b) {
  return key ? key(a) === key(b) : true;
}

export { $PROXY, $RAW, $TARGET, $TRACK, Computation, ContextNotFoundError, NoOwnerError, NotReadyError, Owner, Queue, SUPPORTS_PROXY, createAsync, createBoundary, createContext, createEffect, createErrorBoundary, createMemo, createProjection, createRenderEffect, createRoot, createSignal, createStore, createSuspense, deep, flatten, flushSync, getContext, getObserver, getOwner, hasContext, hasUpdated, isEqual, isPending, isWrappable, latest, mapArray, merge, omit, onCleanup, reconcile, repeat, resolve, runWithObserver, runWithOwner, setContext, tryCatch, untrack, unwrap };
