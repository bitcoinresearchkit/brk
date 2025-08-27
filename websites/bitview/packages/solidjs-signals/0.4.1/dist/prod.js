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
  if (!globalQueue.K)
    queueMicrotask(flush);
}
var pureQueue = [];
var Queue = class {
  o = null;
  K = false;
  L = [[], []];
  F = [];
  created = clock;
  enqueue(type, fn) {
    pureQueue.push(fn);
    if (type)
      this.L[type - 1].push(fn);
    schedule();
  }
  run(type) {
    if (type === EFFECT_PURE) {
      pureQueue.length && runQueue(pureQueue, type);
      pureQueue = [];
      return;
    } else if (this.L[type - 1].length) {
      const effects = this.L[type - 1];
      this.L[type - 1] = [];
      runQueue(effects, type);
    }
    for (let i = 0; i < this.F.length; i++) {
      this.F[i].run(type);
    }
  }
  flush() {
    if (this.K)
      return;
    this.K = true;
    try {
      this.run(EFFECT_PURE);
      incrementClock();
      scheduled = false;
      this.run(EFFECT_RENDER);
      this.run(EFFECT_USER);
    } finally {
      this.K = false;
    }
  }
  addChild(child) {
    this.F.push(child);
    child.o = this;
  }
  removeChild(child) {
    const index = this.F.indexOf(child);
    if (index >= 0)
      this.F.splice(index, 1);
  }
  notify(...args) {
    if (this.o)
      return this.o.notify(...args);
    return false;
  }
};
var globalQueue = new Queue();
function flush() {
  while (scheduled) {
    globalQueue.flush();
  }
}
function runQueue(queue, type) {
  for (let i = 0; i < queue.length; i++)
    queue[i](type);
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
var Owner = class {
  // We flatten the owner tree into a linked list so that we don't need a pointer to .firstChild
  // However, the children are actually added in reverse creation order
  // See comment at the top of the file for an example of the _nextSibling traversal
  o = null;
  m = null;
  t = null;
  a = STATE_CLEAN;
  l = null;
  p = defaultContext;
  h = globalQueue;
  W = 0;
  id = null;
  constructor(id = null, skipAppend = false) {
    this.id = id;
    if (currentOwner) {
      !skipAppend && currentOwner.append(this);
    }
  }
  append(child) {
    child.o = this;
    child.t = this;
    if (this.m)
      this.m.t = child;
    child.m = this.m;
    this.m = child;
    if (this.id != null && child.id == null)
      child.id = this.getNextChildId();
    if (child.p !== this.p) {
      child.p = { ...this.p, ...child.p };
    }
    if (this.h)
      child.h = this.h;
  }
  dispose(self = true) {
    if (this.a === STATE_DISPOSED)
      return;
    let head = self ? this.t || this.o : this, current = this.m, next = null;
    while (current && current.o === this) {
      current.dispose(true);
      current.z();
      next = current.m;
      current.m = null;
      current = next;
    }
    this.W = 0;
    if (self)
      this.z();
    if (current)
      current.t = !self ? this : this.t;
    if (head)
      head.m = current;
  }
  z() {
    if (this.t)
      this.t.m = null;
    this.o = null;
    this.t = null;
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
function formatId(prefix, id) {
  const num = id.toString(36), len = num.length - 1;
  return prefix + (len ? String.fromCharCode(64 + len) : "") + num;
}
function isUndefined(value) {
  return typeof value === "undefined";
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
var unobserved = [];
var notStale = false;
var updateCheck = null;
var staleCheck = null;
function getObserver() {
  return currentObserver;
}
var UNCHANGED = Symbol(0);
var Computation = class extends Owner {
  c = null;
  d = null;
  g;
  G;
  A;
  // Used in __DEV__ mode, hopefully removed in production
  ca;
  // Using false is an optimization as an alternative to _equals: () => false
  // which could enable more efficient DIRTY notification
  S = isEqual;
  X;
  _ = false;
  /** Whether the computation is an error or has ancestors that are unresolved */
  f = 0;
  /** Which flags raised by sources are handled, vs. being passed through. */
  T = DEFAULT_FLAGS;
  B = -1;
  x = false;
  constructor(initialValue, compute2, options) {
    super(options?.id, compute2 === null);
    this.A = compute2;
    this.a = compute2 ? STATE_DIRTY : STATE_CLEAN;
    this.f = compute2 && initialValue === void 0 ? UNINITIALIZED_BIT : 0;
    this.g = initialValue;
    if (options?.equals !== void 0)
      this.S = options.equals;
    if (options?.pureWrite)
      this._ = true;
    if (options?.unobserved)
      this.X = options?.unobserved;
  }
  Y() {
    if (this.A) {
      if (this.f & ERROR_BIT && this.B <= getClock())
        update(this);
      else
        this.y();
    }
    track(this);
    newFlags |= this.f & ~currentMask;
    if (this.f & ERROR_BIT) {
      throw this.G;
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
    if (this.A && this.f & ERROR_BIT && this.B <= getClock()) {
      update(this);
    } else {
      this.y();
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
    const valueChanged = newValue !== UNCHANGED && (!!(this.f & UNINITIALIZED_BIT) || // this._stateFlags & LOADING_BIT & ~flags ||
    this.S === false || !this.S(this.g, newValue));
    if (valueChanged) {
      this.g = newValue;
      this.G = void 0;
    }
    const changedFlagsMask = this.f ^ flags, changedFlags = changedFlagsMask & flags;
    this.f = flags;
    this.B = getClock() + 1;
    if (this.d) {
      for (let i = 0; i < this.d.length; i++) {
        if (valueChanged) {
          this.d[i].r(STATE_DIRTY);
        } else if (changedFlagsMask) {
          this.d[i].Z(changedFlagsMask, changedFlags);
        }
      }
    }
    return this.g;
  }
  /**
   * Set the current node's state, and recursively mark all of this node's observers as STATE_CHECK
   */
  r(state, skipQueue) {
    if (this.a >= state && !this.x)
      return;
    this.x = !!skipQueue;
    this.a = state;
    if (this.d) {
      for (let i = 0; i < this.d.length; i++) {
        this.d[i].r(STATE_CHECK, skipQueue);
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
      if (this.d) {
        for (let i = 0; i < this.d.length; i++) {
          this.d[i].Z(mask, newFlags2);
        }
      }
    }
  }
  M(error) {
    this.G = error;
    this.write(UNCHANGED, this.f & ~LOADING_BIT | ERROR_BIT | UNINITIALIZED_BIT);
  }
  /**
   * This is the core part of the reactivity system, which makes sure that the values are updated
   * before they are read. We've also adapted it to return the loading state of the computation,
   * so that we can propagate that to the computation's observers.
   *
   * This function will ensure that the value and states we read from the computation are up to date
   */
  y() {
    if (!this.A) {
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
        this.c[i].y();
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
  z() {
    if (this.a === STATE_DISPOSED)
      return;
    if (this.c)
      removeSourceObservers(this, 0);
    super.z();
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
      updateCheck.g = computation.B > currentObserver.B;
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
    const result = compute(node, node.A, node);
    node.write(result, newFlags, true);
  } catch (error) {
    if (error instanceof NotReadyError) {
      node.write(UNCHANGED, newFlags | LOADING_BIT | node.f & UNINITIALIZED_BIT);
    } else {
      node.M(error);
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
        if (!source.d)
          source.d = [node];
        else
          source.d.push(node);
      }
    } else if (node.c && newSourcesIndex < node.c.length) {
      removeSourceObservers(node, newSourcesIndex);
      node.c.length = newSourcesIndex;
    }
    unobserved.length && notifyUnobserved();
    newSources = prevSources;
    newSourcesIndex = prevSourcesIndex;
    newFlags = prevFlags;
    node.B = getClock() + 1;
    node.a = STATE_CLEAN;
  }
}
function removeSourceObservers(node, index) {
  let source;
  let swap;
  for (let i = index; i < node.c.length; i++) {
    source = node.c[i];
    if (source.d) {
      swap = source.d.indexOf(node);
      source.d[swap] = source.d[source.d.length - 1];
      source.d.pop();
      if (!source.d.length)
        unobserved.push(source);
    }
  }
}
function notifyUnobserved() {
  for (let i = 0; i < unobserved.length; i++) {
    const source = unobserved[i];
    if (!source.d || !source.d.length)
      unobserved[i].X?.();
  }
  unobserved = [];
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
      observer.M(error);
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
        if (!source.d)
          source.d = [observer];
        else
          source.d.push(observer);
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

// src/core/effect.ts
var Effect = class extends Computation {
  U;
  N;
  C;
  V = false;
  O;
  s;
  constructor(initialValue, compute2, effect, error, options) {
    super(initialValue, compute2, options);
    this.U = effect;
    this.N = error;
    this.O = initialValue;
    this.s = options?.render ? EFFECT_RENDER : EFFECT_USER;
    if (this.s === EFFECT_RENDER) {
      this.A = (p) => getClock() > this.h.created && !(this.f & ERROR_BIT) ? latest(() => compute2(p)) : compute2(p);
    }
    this.y();
    !options?.defer && (this.s === EFFECT_USER ? this.h.enqueue(this.s, this.u.bind(this)) : this.u(this.s));
  }
  write(value, flags = 0) {
    if (this.a == STATE_DIRTY) {
      this.f;
      this.f = flags;
      if (this.s === EFFECT_RENDER) {
        this.h.notify(this, LOADING_BIT | ERROR_BIT, flags);
      }
    }
    if (value === UNCHANGED)
      return this.g;
    this.g = value;
    this.V = true;
    return value;
  }
  r(state, skipQueue) {
    if (this.a >= state || skipQueue)
      return;
    if (this.a === STATE_CLEAN)
      this.h.enqueue(this.s, this.u.bind(this));
    this.a = state;
  }
  M(error) {
    this.G = error;
    this.h.notify(this, LOADING_BIT, 0);
    this.f = ERROR_BIT;
    if (this.s === EFFECT_USER) {
      try {
        return this.N ? this.N(error, () => {
          this.C?.();
          this.C = void 0;
        }) : console.error(error);
      } catch (e) {
        error = e;
      }
    }
    if (!this.h.notify(this, ERROR_BIT, ERROR_BIT))
      throw error;
  }
  z() {
    if (this.a === STATE_DISPOSED)
      return;
    this.U = void 0;
    this.O = void 0;
    this.N = void 0;
    this.C?.();
    this.C = void 0;
    super.z();
  }
  u(type) {
    if (type) {
      if (this.V && this.a !== STATE_DISPOSED) {
        this.C?.();
        try {
          this.C = this.U(this.g, this.O);
        } catch (e) {
          if (!this.h.notify(this, ERROR_BIT, ERROR_BIT))
            throw e;
        } finally {
          this.O = this.g;
          this.V = false;
        }
      }
    } else
      this.a !== STATE_CLEAN && runTop(this);
  }
};
var EagerComputation = class extends Computation {
  constructor(initialValue, compute2, options) {
    super(initialValue, compute2, options);
    !options?.defer && this.y();
  }
  r(state, skipQueue) {
    if (this.a >= state && !this.x)
      return;
    if (!skipQueue && (this.a === STATE_CLEAN || this.a === STATE_CHECK && this.x))
      this.h.enqueue(EFFECT_PURE, this.u.bind(this));
    super.r(state, skipQueue);
  }
  u() {
    this.a !== STATE_CLEAN && runTop(this);
  }
};
var FirewallComputation = class extends Computation {
  firewall = true;
  constructor(compute2) {
    super(void 0, compute2);
  }
  r(state, skipQueue) {
    if (this.a >= state && !this.x)
      return;
    if (!skipQueue && (this.a === STATE_CLEAN || this.a === STATE_CHECK && this.x))
      this.h.enqueue(EFFECT_PURE, this.u.bind(this));
    super.r(state, true);
    this.x = !!skipQueue;
  }
  u() {
    this.a !== STATE_CLEAN && runTop(this);
  }
};
function runTop(node) {
  const ancestors = [];
  for (let current = node; current !== null; current = current.o) {
    if (current.a !== STATE_CLEAN) {
      ancestors.push(current);
    }
  }
  for (let i = ancestors.length - 1; i >= 0; i--) {
    if (ancestors[i].a !== STATE_DISPOSED)
      ancestors[i].y();
  }
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
  const o = getOwner();
  const needsId = o?.id != null;
  const node = new Computation(
    first,
    null,
    needsId ? { id: o.getNextChildId(), ...second } : second
  );
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
  let refreshing = false;
  const node = new EagerComputation(
    value,
    (p) => {
      const source = compute2(p, refreshing);
      refreshing = false;
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
            node.M(error);
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
  const read = node.wait.bind(node);
  read.refresh = () => {
    node.a = STATE_DIRTY;
    refreshing = true;
    node.y();
  };
  return read;
}
function createEffect(compute2, effect, value, options) {
  void new Effect(
    value,
    compute2,
    effect.effect ? effect.effect : effect,
    effect.error,
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
function transition(fn) {
}
function createOptimistic(initial, compute2, options) {
  return [];
}

// src/store/projection.ts
function createProjection(fn, initialValue = {}) {
  let wrappedStore;
  const node = new FirewallComputation(() => {
    storeSetter(wrappedStore, fn);
  });
  const wrappedMap = /* @__PURE__ */ new WeakMap();
  const traps = {
    ...storeTraps,
    get(target, property, receiver) {
      const o = getOwner();
      (!o || o !== node) && node.wait();
      return storeTraps.get(target, property, receiver);
    }
  };
  function wrapProjection(source) {
    if (wrappedMap.has(source))
      return wrappedMap.get(source);
    if (source[$TARGET]?.[STORE_WRAP] === wrapProjection)
      return source;
    const wrapped = createStoreProxy(source, traps, {
      [STORE_WRAP]: wrapProjection,
      [STORE_LOOKUP]: wrappedMap
    });
    wrappedMap.set(source, wrapped);
    return wrapped;
  }
  return wrappedStore = wrapProjection(initialValue);
}

// src/store/store.ts
var $TRACK = Symbol(0);
var $DEEP = Symbol(0);
var $TARGET = Symbol(0);
var $PROXY = Symbol(0);
var $DELETED = Symbol(0);
var PARENTS = /* @__PURE__ */ new WeakMap();
var STORE_VALUE = "v";
var STORE_OVERRIDE = "o";
var STORE_NODE = "n";
var STORE_HAS = "h";
var STORE_WRAP = "w";
var STORE_LOOKUP = "l";
function createStoreProxy(value, traps = storeTraps, extend) {
  let newTarget;
  if (Array.isArray(value)) {
    newTarget = [];
    newTarget.v = value;
  } else
    newTarget = { v: value };
  extend && Object.assign(newTarget, extend);
  return newTarget[$PROXY] = new Proxy(newTarget, traps);
}
var storeLookup = /* @__PURE__ */ new WeakMap();
function wrap(value, target) {
  if (target?.[STORE_WRAP])
    return target[STORE_WRAP](value, target);
  let p = value[$PROXY] || storeLookup.get(value);
  if (!p)
    storeLookup.set(value, p = createStoreProxy(value));
  return p;
}
function isWrappable(obj) {
  return obj != null && typeof obj === "object" && !Object.isFrozen(obj);
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
function trackSelf(target, symbol = $TRACK) {
  getObserver() && getNode(getNodes(target, STORE_NODE), symbol, void 0, false).read();
}
function getKeys(source, override, enumerable = true) {
  const baseKeys = untrack(() => enumerable ? Object.keys(source) : Reflect.ownKeys(source));
  if (!override)
    return baseKeys;
  const keys = new Set(baseKeys);
  const overrides = Reflect.ownKeys(override);
  for (const key of overrides) {
    if (override[key] !== $DELETED)
      keys.add(key);
    else
      keys.delete(key);
  }
  return Array.from(keys);
}
function getPropertyDescriptor(source, override, property) {
  let value = source;
  if (override && property in override) {
    if (value[property] === $DELETED)
      return void 0;
    if (!(property in value))
      value = override;
  }
  return Reflect.getOwnPropertyDescriptor(value, property);
}
var Writing = null;
var storeTraps = {
  get(target, property, receiver) {
    if (property === $TARGET)
      return target;
    if (property === $PROXY)
      return receiver;
    if (property === $TRACK || property === $DEEP) {
      trackSelf(target, property);
      return receiver;
    }
    const nodes = getNodes(target, STORE_NODE);
    const tracked = nodes[property];
    const overridden = target[STORE_OVERRIDE] && property in target[STORE_OVERRIDE];
    const proxySource = !!target[STORE_VALUE][$TARGET];
    const storeValue = overridden ? target[STORE_OVERRIDE] : target[STORE_VALUE];
    if (!tracked) {
      const desc = Object.getOwnPropertyDescriptor(storeValue, property);
      if (desc && desc.get)
        return desc.get.call(receiver);
    }
    if (Writing?.has(receiver)) {
      let value2 = tracked && (overridden || !proxySource) ? tracked.g : storeValue[property];
      value2 === $DELETED && (value2 = void 0);
      if (!isWrappable(value2))
        return value2;
      const wrapped = wrap(value2, target);
      Writing.add(wrapped);
      return wrapped;
    }
    let value = tracked ? overridden || !proxySource ? nodes[property].read() : (nodes[property].read(), storeValue[property]) : storeValue[property];
    value === $DELETED && (value = void 0);
    if (!tracked) {
      if (!overridden && typeof value === "function" && !storeValue.hasOwnProperty(property)) {
        let proto;
        return !Array.isArray(target[STORE_VALUE]) && (proto = Object.getPrototypeOf(target[STORE_VALUE])) && proto !== Object.prototype ? value.bind(storeValue) : value;
      } else if (getObserver()) {
        return getNode(nodes, property, isWrappable(value) ? wrap(value, target) : value).read();
      }
    }
    return isWrappable(value) ? wrap(value, target) : value;
  },
  has(target, property) {
    if (property === $PROXY || property === $TRACK || property === "__proto__")
      return true;
    const has = target[STORE_OVERRIDE] && property in target[STORE_OVERRIDE] ? target[STORE_OVERRIDE][property] !== $DELETED : property in target[STORE_VALUE];
    getObserver() && getNode(getNodes(target, STORE_HAS), property, has).read();
    return has;
  },
  set(target, property, rawValue) {
    const store = target[$PROXY];
    if (Writing?.has(target[$PROXY])) {
      untrack(() => {
        const state = target[STORE_VALUE];
        const base = state[property];
        const prev = target[STORE_OVERRIDE]?.[property] || base;
        const value = rawValue?.[$TARGET]?.[STORE_VALUE] ?? rawValue;
        if (prev === value)
          return true;
        const len = target[STORE_OVERRIDE]?.length || state.length;
        if (value !== void 0 && value === base)
          delete target[STORE_OVERRIDE][property];
        else
          (target[STORE_OVERRIDE] || (target[STORE_OVERRIDE] = /* @__PURE__ */ Object.create(null)))[property] = value;
        const wrappable = isWrappable(value);
        if (isWrappable(prev)) {
          const parents = PARENTS.get(prev);
          parents && (parents instanceof Set ? parents.delete(store) : PARENTS.delete(prev));
        }
        if (recursivelyNotify(store, storeLookup) && wrappable)
          recursivelyAddParent(value, store);
        target[STORE_HAS]?.[property]?.write(true);
        const nodes = getNodes(target, STORE_NODE);
        nodes[property]?.write(wrappable ? wrap(value, target) : value);
        if (Array.isArray(state)) {
          const index = parseInt(property) + 1;
          if (index > len)
            nodes.length?.write(index);
        }
        nodes[$TRACK]?.write(void 0);
      });
    }
    return true;
  },
  deleteProperty(target, property) {
    if (Writing?.has(target[$PROXY]) && target[STORE_OVERRIDE]?.[property] !== $DELETED) {
      untrack(() => {
        const prev = target[STORE_OVERRIDE]?.[property] || target[STORE_VALUE][property];
        if (property in target[STORE_VALUE]) {
          (target[STORE_OVERRIDE] || (target[STORE_OVERRIDE] = /* @__PURE__ */ Object.create(null)))[property] = $DELETED;
        } else if (target[STORE_OVERRIDE] && property in target[STORE_OVERRIDE]) {
          delete target[STORE_OVERRIDE][property];
        } else
          return true;
        if (isWrappable(prev)) {
          const parents = PARENTS.get(prev);
          parents && (parents instanceof Set ? parents.delete(target) : PARENTS.delete(prev));
        }
        target[STORE_HAS]?.[property]?.write(false);
        const nodes = getNodes(target, STORE_NODE);
        nodes[property]?.write(void 0);
        nodes[$TRACK]?.write(void 0);
      });
    }
    return true;
  },
  ownKeys(target) {
    trackSelf(target);
    return getKeys(target[STORE_VALUE], target[STORE_OVERRIDE], false);
  },
  getOwnPropertyDescriptor(target, property) {
    if (property === $PROXY)
      return { value: target[$PROXY], writable: true, configurable: true };
    return getPropertyDescriptor(target[STORE_VALUE], target[STORE_OVERRIDE], property);
  },
  getPrototypeOf(target) {
    return Object.getPrototypeOf(target[STORE_VALUE]);
  }
};
function storeSetter(store, fn) {
  const prevWriting = Writing;
  Writing = /* @__PURE__ */ new Set();
  Writing.add(store);
  try {
    fn(store);
  } finally {
    Writing.clear();
    Writing = prevWriting;
  }
}
function createStore(first, second) {
  const derived = typeof first === "function", wrappedStore = derived ? createProjection(first, second) : wrap(first);
  return [wrappedStore, (fn) => storeSetter(wrappedStore, fn)];
}
function recursivelyNotify(state, lookup) {
  let target = state[$TARGET] || lookup?.get(state)?.[$TARGET];
  let notified = false;
  if (target) {
    const deep2 = getNodes(target, STORE_NODE)[$DEEP];
    if (deep2) {
      deep2.write(void 0);
      notified = true;
    }
    lookup = target[STORE_LOOKUP] || lookup;
  }
  const parents = PARENTS.get(target?.[STORE_VALUE] || state);
  if (!parents)
    return notified;
  if (parents instanceof Set) {
    for (let parent of parents)
      notified = recursivelyNotify(parent, lookup) || notified;
  } else
    notified = recursivelyNotify(parents, lookup) || notified;
  return notified;
}
function recursivelyAddParent(state, parent) {
  let override;
  const target = state[$TARGET];
  if (target) {
    override = target[STORE_OVERRIDE];
    state = target[STORE_VALUE];
  }
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
    const len = override?.length || state.length;
    for (let i = 0; i < len; i++) {
      const item = override && i in override ? override[i] : state[i];
      isWrappable(item) && recursivelyAddParent(item, state);
    }
  } else {
    const keys = getKeys(state, override);
    for (let i = 0; i < keys.length; i++) {
      const key = keys[i];
      const item = override && key in override ? override[key] : state[key];
      isWrappable(item) && recursivelyAddParent(item, state);
    }
  }
}
function deep(store) {
  recursivelyAddParent(store);
  return store[$DEEP];
}

// src/store/reconcile.ts
function unwrap(value) {
  return value?.[$TARGET]?.[STORE_NODE] ?? value;
}
function getOverrideValue(value, override, key) {
  return override && key in override ? override[key] : value[key];
}
function getAllKeys(value, override, next) {
  const keys = getKeys(value, override);
  const nextKeys = Object.keys(next);
  return Array.from(/* @__PURE__ */ new Set([...keys, ...nextKeys]));
}
function applyState(next, state, keyFn, all) {
  const target = state?.[$TARGET];
  if (!target)
    return;
  const previous = target[STORE_VALUE];
  const override = target[STORE_OVERRIDE];
  if (next === previous && !override)
    return;
  (target[STORE_LOOKUP] || storeLookup).set(next, target[$PROXY]);
  target[STORE_VALUE] = next;
  target[STORE_OVERRIDE] = void 0;
  if (Array.isArray(previous)) {
    let changed = false;
    const prevLength = getOverrideValue(previous, override, "length");
    if (next.length && prevLength && next[0] && keyFn(next[0]) != null) {
      let i, j, start, end, newEnd, item, newIndicesNext, keyVal;
      for (start = 0, end = Math.min(prevLength, next.length); start < end && ((item = getOverrideValue(previous, override, start)) === next[start] || item && next[start] && keyFn(item) === keyFn(next[start])); start++) {
        applyState(next[start], wrap(item, target), keyFn, all);
      }
      const temp = new Array(next.length), newIndices = /* @__PURE__ */ new Map();
      for (end = prevLength - 1, newEnd = next.length - 1; end >= start && newEnd >= start && ((item = getOverrideValue(previous, override, end)) === next[newEnd] || item && next[newEnd] && keyFn(item) === keyFn(next[newEnd])); end--, newEnd--) {
        temp[newEnd] = item;
      }
      if (start > newEnd || start > end) {
        for (j = start; j <= newEnd; j++) {
          changed = true;
          target[STORE_NODE][j]?.write(wrap(next[j], target));
        }
        for (; j < next.length; j++) {
          changed = true;
          const wrapped = wrap(temp[j], target);
          target[STORE_NODE][j]?.write(wrapped);
          applyState(next[j], wrapped, keyFn, all);
        }
        changed && target[STORE_NODE][$TRACK]?.write(void 0);
        prevLength !== next.length && target[STORE_NODE].length?.write(next.length);
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
        item = getOverrideValue(previous, override, i);
        keyVal = item ? keyFn(item) : item;
        j = newIndices.get(keyVal);
        if (j !== void 0 && j !== -1) {
          temp[j] = item;
          j = newIndicesNext[j];
          newIndices.set(keyVal, j);
        }
      }
      for (j = start; j < next.length; j++) {
        if (j in temp) {
          const wrapped = wrap(temp[j], target);
          target[STORE_NODE][j]?.write(wrapped);
          applyState(next[j], wrapped, keyFn, all);
        } else
          target[STORE_NODE][j]?.write(wrap(next[j], target));
      }
      if (start < next.length)
        changed = true;
    } else if (prevLength && next.length) {
      for (let i = 0, len = next.length; i < len; i++) {
        const item = getOverrideValue(previous, override, i);
        isWrappable(item) && applyState(next[i], wrap(item, target), keyFn, all);
      }
    }
    if (prevLength !== next.length) {
      changed = true;
      target[STORE_NODE].length?.write(next.length);
    }
    changed && target[STORE_NODE][$TRACK]?.write(void 0);
    return;
  }
  let nodes = target[STORE_NODE];
  if (nodes) {
    const tracked = nodes[$TRACK];
    const keys = tracked || all ? getAllKeys(previous, override, next) : Object.keys(nodes);
    for (let i = 0, len = keys.length; i < len; i++) {
      const key = keys[i];
      const node = nodes[key];
      const previousValue = unwrap(getOverrideValue(previous, override, key));
      let nextValue = unwrap(next[key]);
      if (previousValue === nextValue)
        continue;
      if (!previousValue || !isWrappable(previousValue) || keyFn(previousValue) != null && keyFn(previousValue) !== keyFn(nextValue)) {
        tracked?.write(void 0);
        node?.write(isWrappable(nextValue) ? wrap(nextValue, target) : nextValue);
      } else
        applyState(nextValue, wrap(previousValue, target), keyFn, all);
    }
  }
  if (nodes = target[STORE_HAS]) {
    const keys = Object.keys(nodes);
    for (let i = 0, len = keys.length; i < len; i++) {
      nodes[keys[i]].write(keys[i] in next);
    }
  }
}
function reconcile(value, key, all = false) {
  return (state) => {
    const keyFn = typeof key === "string" ? (item) => item[key] : key;
    const eq = keyFn(state);
    if (eq !== void 0 && keyFn(value) !== keyFn(state))
      throw new Error("Cannot reconcile states with different identity");
    applyState(value, state, keyFn, all);
  };
}

// src/store/utils.ts
function snapshot(item, map, lookup) {
  let target, isArray, override, result, unwrapped, v;
  if (!isWrappable(item))
    return item;
  if (map && map.has(item))
    return map.get(item);
  if (!map)
    map = /* @__PURE__ */ new Map();
  if (target = item[$TARGET] || lookup?.get(item)?.[$TARGET]) {
    override = target[STORE_OVERRIDE];
    isArray = Array.isArray(target[STORE_VALUE]);
    map.set(
      item,
      override ? result = isArray ? [] : Object.create(Object.getPrototypeOf(target[STORE_VALUE])) : target[STORE_VALUE]
    );
    item = target[STORE_VALUE];
    lookup = storeLookup;
  } else {
    isArray = Array.isArray(item);
    map.set(item, item);
  }
  if (isArray) {
    const len = override?.length || item.length;
    for (let i = 0; i < len; i++) {
      v = override && i in override ? override[i] : item[i];
      if (v === $DELETED)
        continue;
      if ((unwrapped = snapshot(v, map, lookup)) !== v || result) {
        if (!result)
          map.set(item, result = [...item]);
        result[i] = unwrapped;
      }
    }
  } else {
    const keys = getKeys(item, override);
    for (let i = 0, l = keys.length; i < l; i++) {
      let prop = keys[i];
      const desc = getPropertyDescriptor(item, override, prop);
      if (desc.get)
        continue;
      v = override && prop in override ? override[prop] : item[prop];
      if ((unwrapped = snapshot(v, map, lookup)) !== item[prop] || result) {
        if (!result) {
          result = Object.create(Object.getPrototypeOf(item));
          Object.assign(result, item);
        }
        result[prop] = unwrapped;
      }
    }
  }
  return result || item;
}
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
    $: list,
    w: [],
    D: map,
    e: [],
    b: [],
    E: keyFn,
    j: keyFn || options?.keyed === false ? [] : void 0,
    k: map.length > 1 ? [] : void 0,
    I: options?.fallback
  });
}
var pureOptions = { pureWrite: true };
function updateKeyedMap() {
  const newItems = this.$() || [], newLen = newItems.length;
  newItems[$TRACK];
  runWithOwner(this.H, () => {
    let i, j, mapper = this.j ? () => {
      this.j[j] = new Computation(newItems[j], null, pureOptions);
      this.k && (this.k[j] = new Computation(j, null, pureOptions));
      return this.D(
        Computation.prototype.read.bind(this.j[j]),
        this.k ? Computation.prototype.read.bind(this.k[j]) : void 0
      );
    } : this.k ? () => {
      const item = newItems[j];
      this.k[j] = new Computation(j, null, pureOptions);
      return this.D(() => item, Computation.prototype.read.bind(this.k[j]));
    } : () => {
      const item = newItems[j];
      return this.D(() => item);
    };
    if (newLen === 0) {
      if (this.i !== 0) {
        this.H.dispose(false);
        this.b = [];
        this.w = [];
        this.e = [];
        this.i = 0;
        this.j && (this.j = []);
        this.k && (this.k = []);
      }
      if (this.I && !this.e[0]) {
        this.e[0] = compute(
          this.b[0] = new Owner(),
          this.I,
          null
        );
      }
    } else if (this.i === 0) {
      if (this.b[0])
        this.b[0].dispose();
      this.e = new Array(newLen);
      for (j = 0; j < newLen; j++) {
        this.w[j] = newItems[j];
        this.e[j] = compute(this.b[j] = new Owner(), mapper, null);
      }
      this.i = newLen;
    } else {
      let start, end, newEnd, item, key, newIndices, newIndicesNext, temp = new Array(newLen), tempNodes = new Array(newLen), tempRows = this.j ? new Array(newLen) : void 0, tempIndexes = this.k ? new Array(newLen) : void 0;
      for (start = 0, end = Math.min(this.i, newLen); start < end && (this.w[start] === newItems[start] || this.j && compare(this.E, this.w[start], newItems[start])); start++) {
        if (this.j)
          this.j[start].write(newItems[start]);
      }
      for (end = this.i - 1, newEnd = newLen - 1; end >= start && newEnd >= start && (this.w[end] === newItems[newEnd] || this.j && compare(this.E, this.w[end], newItems[newEnd])); end--, newEnd--) {
        temp[newEnd] = this.e[end];
        tempNodes[newEnd] = this.b[end];
        tempRows && (tempRows[newEnd] = this.j[end]);
        tempIndexes && (tempIndexes[newEnd] = this.k[end]);
      }
      newIndices = /* @__PURE__ */ new Map();
      newIndicesNext = new Array(newEnd + 1);
      for (j = newEnd; j >= start; j--) {
        item = newItems[j];
        key = this.E ? this.E(item) : item;
        i = newIndices.get(key);
        newIndicesNext[j] = i === void 0 ? -1 : i;
        newIndices.set(key, j);
      }
      for (i = start; i <= end; i++) {
        item = this.w[i];
        key = this.E ? this.E(item) : item;
        j = newIndices.get(key);
        if (j !== void 0 && j !== -1) {
          temp[j] = this.e[i];
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
          this.e[j] = temp[j];
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
          this.e[j] = compute(this.b[j] = new Owner(), mapper, null);
        }
      }
      this.e = this.e.slice(0, this.i = newLen);
      this.w = newItems.slice(0);
    }
  });
  return this.e;
}
function repeat(count, map, options) {
  return updateRepeat.bind({
    H: new Owner(),
    i: 0,
    q: 0,
    aa: count,
    D: map,
    b: [],
    e: [],
    ba: options?.from,
    I: options?.fallback
  });
}
function updateRepeat() {
  const newLen = this.aa();
  const from = this.ba?.() || 0;
  runWithOwner(this.H, () => {
    if (newLen === 0) {
      if (this.i !== 0) {
        this.H.dispose(false);
        this.b = [];
        this.e = [];
        this.i = 0;
      }
      if (this.I && !this.e[0]) {
        this.e[0] = compute(
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
      this.e.splice(0, from - this.q);
    } else if (this.q > from) {
      let i = prevTo - this.q - 1;
      let difference = this.q - from;
      this.b.length = this.e.length = newLen;
      while (i >= difference) {
        this.b[i] = this.b[i - difference];
        this.e[i] = this.e[i - difference];
        i--;
      }
      for (let i2 = 0; i2 < difference; i2++) {
        this.e[i2] = compute(
          this.b[i2] = new Owner(),
          () => this.D(i2 + from),
          null
        );
      }
    }
    for (let i = prevTo; i < to; i++) {
      this.e[i - from] = compute(
        this.b[i - from] = new Owner(),
        () => this.D(i),
        null
      );
    }
    this.e = this.e.slice(0, newLen);
    this.q = from;
    this.i = newLen;
  });
  return this.e;
}
function compare(key, a, b) {
  return key ? key(a) === key(b) : true;
}

// src/boundaries.ts
var BoundaryComputation = class extends EagerComputation {
  J;
  constructor(compute2, propagationMask) {
    super(void 0, compute2, { defer: true });
    this.J = propagationMask;
  }
  write(value, flags) {
    super.write(value, flags & ~this.J);
    if (this.J & LOADING_BIT && !(this.f & UNINITIALIZED_BIT)) {
      flags &= ~LOADING_BIT;
    }
    this.h.notify(this, this.J, flags);
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
      if (type & LOADING_BIT) {
        if (flags & LOADING_BIT) {
          this.Q.add(node);
          type &= ~LOADING_BIT;
        } else if (this.Q.delete(node))
          type &= ~LOADING_BIT;
      }
      if (type & ERROR_BIT) {
        if (flags & ERROR_BIT) {
          this.P.add(node);
          type &= ~ERROR_BIT;
        } else if (this.P.delete(node))
          type &= ~ERROR_BIT;
      }
    }
    return type ? super.notify(node, type, flags) : true;
  }
};
var CollectionQueue = class extends Queue {
  R;
  b = /* @__PURE__ */ new Set();
  n = new Computation(false, null, { pureWrite: true });
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
  const queue = new ConditionalQueue(
    new Computation(void 0, () => condition() === "hidden" /* HIDDEN */)
  );
  const tree = createBoundChildren(owner, fn, queue, 0);
  new EagerComputation(void 0, () => {
    const disabled = queue.n.read();
    tree.J = disabled ? ERROR_BIT | LOADING_BIT : 0;
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
    (queue) => fallback(queue.b.values().next().value.G, () => {
      incrementClock();
      for (let node of queue.b) {
        node.a = STATE_DIRTY;
        node.h?.enqueue(node.s, node.u.bind(node));
      }
    })
  );
}
function flatten(children, options) {
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

export { $PROXY, $TARGET, $TRACK, Computation, ContextNotFoundError, NoOwnerError, NotReadyError, Owner, Queue, SUPPORTS_PROXY, createAsync, createBoundary, createContext, createEffect, createErrorBoundary, createMemo, createOptimistic, createProjection, createRenderEffect, createRoot, createSignal, createStore, createSuspense, deep, flatten, flush, getContext, getObserver, getOwner, hasContext, hasUpdated, isEqual, isPending, isWrappable, latest, mapArray, merge, omit, onCleanup, reconcile, repeat, resolve, runWithObserver, runWithOwner, setContext, snapshot, transition, tryCatch, untrack };
