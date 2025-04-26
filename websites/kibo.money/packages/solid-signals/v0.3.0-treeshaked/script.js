// @ts-nocheck
// src/core/error.ts
var NotReadyError = class extends Error {
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
  if (!globalQueue.y)
    queueMicrotask(flushSync);
}
var Queue = class {
  i = null;
  y = false;
  m = [[], [], []];
  v = [];
  created = clock;
  enqueue(type, node) {
    this.m[0].push(node);
    if (type)
      this.m[type].push(node);
    schedule();
  }
  run(type) {
    if (this.m[type].length) {
      if (type === EFFECT_PURE) {
        runPureQueue(this.m[type]);
        this.m[type] = [];
      } else {
        const effects = this.m[type];
        this.m[type] = [];
        runEffectQueue(effects);
      }
    }
    let rerun = false;
    for (let i = 0; i < this.v.length; i++) {
      rerun = this.v[i].run(type) || rerun;
    }
    if (type === EFFECT_PURE)
      return rerun || !!this.m[type].length;
  }
  flush() {
    if (this.y)
      return;
    this.y = true;
    try {
      while (this.run(EFFECT_PURE)) {
      }
      incrementClock();
      scheduled = false;
      this.run(EFFECT_RENDER);
      this.run(EFFECT_USER);
    } finally {
      this.y = false;
    }
  }
  addChild(child) {
    this.v.push(child);
    child.i = this;
  }
  removeChild(child) {
    const index = this.v.indexOf(child);
    if (index >= 0)
      this.v.splice(index, 1);
  }
  notify(...args) {
    if (this.i)
      return this.i.notify(...args);
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
  for (let current = node; current !== null; current = current.i) {
    if (current.a !== STATE_CLEAN) {
      ancestors.push(current);
    }
  }
  for (let i = ancestors.length - 1; i >= 0; i--) {
    if (ancestors[i].a !== STATE_DISPOSED)
      ancestors[i].p();
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
    queue[i].L();
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
  i = null;
  g = null;
  n = null;
  a = STATE_CLEAN;
  h = null;
  j = defaultContext;
  f = globalQueue;
  G = null;
  M = 0;
  id = null;
  constructor(id = null, skipAppend = false) {
    this.id = id;
    if (currentOwner && !skipAppend)
      currentOwner.append(this);
  }
  append(child) {
    child.i = this;
    child.n = this;
    if (this.id) {
      child.G = this.g ? this.g.G + 1 : 0;
      child.id = formatId(this.id, child.G);
    }
    if (this.g)
      this.g.n = child;
    child.g = this.g;
    this.g = child;
    if (child.j !== this.j) {
      child.j = { ...this.j, ...child.j };
    }
    if (this.f)
      child.f = this.f;
  }
  dispose(self = true) {
    if (this.a === STATE_DISPOSED)
      return;
    let head = self ? this.n || this.i : this, current = this.g, next = null;
    while (current && current.i === this) {
      current.dispose(true);
      current.q();
      next = current.g;
      current.g = null;
      current = next;
    }
    this.M = 0;
    if (self)
      this.q();
    if (current)
      current.n = !self ? this : this.n;
    if (head)
      head.g = current;
  }
  q() {
    if (this.n)
      this.n.g = null;
    this.i = null;
    this.n = null;
    this.j = defaultContext;
    this.a = STATE_DISPOSED;
    this.emptyDisposal();
  }
  emptyDisposal() {
    if (!this.h)
      return;
    if (Array.isArray(this.h)) {
      for (let i = 0; i < this.h.length; i++) {
        const callable = this.h[i];
        callable.call(callable);
      }
    } else {
      this.h.call(this.h);
    }
    this.h = null;
  }
  getNextChildId() {
    if (this.id)
      return formatId(this.id + "-", this.M++);
    throw new Error("Cannot get child id from owner without an id");
  }
};
function onCleanup(fn) {
  if (!currentOwner)
    return fn;
  const node = currentOwner;
  if (!node.h) {
    node.h = fn;
  } else if (Array.isArray(node.h)) {
    node.h.push(fn);
  } else {
    node.h = [node.h, fn];
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
var UNCHANGED = Symbol(0);
var Computation = class extends Owner {
  b = null;
  c = null;
  e;
  w;
  r;
  // Used in __DEV__ mode, hopefully removed in production
  Q;
  // Using false is an optimization as an alternative to _equals: () => false
  // which could enable more efficient DIRTY notification
  H = isEqual;
  N;
  /** Whether the computation is an error or has ancestors that are unresolved */
  d = 0;
  /** Which flags raised by sources are handled, vs. being passed through. */
  I = DEFAULT_FLAGS;
  s = -1;
  z = false;
  constructor(initialValue, compute2, options) {
    super(null, compute2 === null);
    this.r = compute2;
    this.a = compute2 ? STATE_DIRTY : STATE_CLEAN;
    this.d = compute2 && initialValue === void 0 ? UNINITIALIZED_BIT : 0;
    this.e = initialValue;
    if (options?.equals !== void 0)
      this.H = options.equals;
    if (options?.unobserved)
      this.N = options?.unobserved;
  }
  O() {
    if (this.r) {
      if (this.d & ERROR_BIT && this.s <= getClock())
        update(this);
      else
        this.p();
    }
    track(this);
    newFlags |= this.d & ~currentMask;
    if (this.d & ERROR_BIT) {
      throw this.w;
    } else {
      return this.e;
    }
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   */
  read() {
    return this.O();
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   *
   * If the computation has any unresolved ancestors, this function waits for the value to resolve
   * before continuing
   */
  wait() {
    if (this.r && this.d & ERROR_BIT && this.s <= getClock()) {
      update(this);
    } else {
      this.p();
    }
    track(this);
    if ((notStale || this.d & UNINITIALIZED_BIT) && this.d & LOADING_BIT) {
      throw new NotReadyError();
    }
    return this.O();
  }
  /** Update the computation with a new value. */
  write(value, flags = 0, raw = false) {
    const newValue = !raw && typeof value === "function" ? value(this.e) : value;
    const valueChanged = newValue !== UNCHANGED && (!!(this.d & UNINITIALIZED_BIT) || this.d & LOADING_BIT & ~flags || this.H === false || !this.H(this.e, newValue));
    if (valueChanged) {
      this.e = newValue;
      this.w = void 0;
    }
    const changedFlagsMask = this.d ^ flags, changedFlags = changedFlagsMask & flags;
    this.d = flags;
    this.s = getClock() + 1;
    if (this.c) {
      for (let i = 0; i < this.c.length; i++) {
        if (valueChanged) {
          this.c[i].l(STATE_DIRTY);
        } else if (changedFlagsMask) {
          this.c[i].P(changedFlagsMask, changedFlags);
        }
      }
    }
    return this.e;
  }
  /**
   * Set the current node's state, and recursively mark all of this node's observers as STATE_CHECK
   */
  l(state, skipQueue) {
    if (this.a >= state && !this.z)
      return;
    this.z = !!skipQueue;
    this.a = state;
    if (this.c) {
      for (let i = 0; i < this.c.length; i++) {
        this.c[i].l(STATE_CHECK, skipQueue);
      }
    }
  }
  /**
   * Notify the computation that one of its sources has changed flags.
   *
   * @param mask A bitmask for which flag(s) were changed.
   * @param newFlags The source's new flags, masked to just the changed ones.
   */
  P(mask, newFlags2) {
    if (this.a >= STATE_DIRTY)
      return;
    if (mask & this.I) {
      this.l(STATE_DIRTY);
      return;
    }
    if (this.a >= STATE_CHECK)
      return;
    const prevFlags = this.d & mask;
    const deltaFlags = prevFlags ^ newFlags2;
    if (newFlags2 === prevFlags) ; else if (deltaFlags & prevFlags & mask) {
      this.l(STATE_CHECK);
    } else {
      this.d ^= deltaFlags;
      if (this.c) {
        for (let i = 0; i < this.c.length; i++) {
          this.c[i].P(mask, newFlags2);
        }
      }
    }
  }
  J(error) {
    this.w = error;
    this.write(UNCHANGED, this.d & ~LOADING_BIT | ERROR_BIT | UNINITIALIZED_BIT);
  }
  /**
   * This is the core part of the reactivity system, which makes sure that the values are updated
   * before they are read. We've also adapted it to return the loading state of the computation,
   * so that we can propagate that to the computation's observers.
   *
   * This function will ensure that the value and states we read from the computation are up to date
   */
  p() {
    if (!this.r) {
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
      for (let i = 0; i < this.b.length; i++) {
        this.b[i].p();
        observerFlags |= this.b[i].d;
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
  q() {
    if (this.a === STATE_DISPOSED)
      return;
    if (this.b)
      removeSourceObservers(this, 0);
    super.q();
  }
};
function track(computation) {
  if (currentObserver) {
    if (!newSources && currentObserver.b && currentObserver.b[newSourcesIndex] === computation) {
      newSourcesIndex++;
    } else if (!newSources)
      newSources = [computation];
    else if (computation !== newSources[newSources.length - 1]) {
      newSources.push(computation);
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
    const result = compute(node, node.r, node);
    node.write(result, newFlags, true);
  } catch (error) {
    if (error instanceof NotReadyError) {
      node.write(UNCHANGED, newFlags | LOADING_BIT | node.d & UNINITIALIZED_BIT);
    } else {
      node.J(error);
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
    node.s = getClock() + 1;
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
        source.N?.();
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
function compute(owner, fn, observer) {
  const prevOwner = setOwner(owner), prevObserver = currentObserver, prevMask = currentMask, prevNotStale = notStale;
  currentObserver = observer;
  currentMask = observer?.I ?? DEFAULT_FLAGS;
  notStale = true;
  try {
    return fn(observer ? observer.e : void 0);
  } finally {
    setOwner(prevOwner);
    currentObserver = prevObserver;
    currentMask = prevMask;
    notStale = prevNotStale;
  }
}

// src/core/effect.ts
var Effect = class extends Computation {
  A;
  B;
  t;
  K = false;
  C;
  o;
  constructor(initialValue, compute2, effect, error, options) {
    super(initialValue, compute2, options);
    this.A = effect;
    this.B = error;
    this.C = initialValue;
    this.o = options?.render ? EFFECT_RENDER : EFFECT_USER;
    if (this.o === EFFECT_RENDER) {
      this.r = (p) => getClock() > this.f.created && !(this.d & ERROR_BIT) ? latest(() => compute2(p)) : compute2(p);
    }
    this.p();
    !options?.defer && (this.o === EFFECT_USER ? this.f.enqueue(this.o, this) : this.L());
  }
  write(value, flags = 0) {
    if (this.a == STATE_DIRTY) {
      this.d;
      this.d = flags;
      if (this.o === EFFECT_RENDER) {
        this.f.notify(this, LOADING_BIT | ERROR_BIT, flags);
      }
    }
    if (value === UNCHANGED)
      return this.e;
    this.e = value;
    this.K = true;
    return value;
  }
  l(state, skipQueue) {
    if (this.a >= state || skipQueue)
      return;
    if (this.a === STATE_CLEAN)
      this.f.enqueue(this.o, this);
    this.a = state;
  }
  J(error) {
    this.w = error;
    this.t?.();
    this.f.notify(this, LOADING_BIT, 0);
    this.d = ERROR_BIT;
    if (this.o === EFFECT_USER) {
      try {
        return this.B ? this.t = this.B(error) : console.error(new EffectError(this.A, error));
      } catch (e) {
        error = e;
      }
    }
    if (!this.f.notify(this, ERROR_BIT, ERROR_BIT))
      throw error;
  }
  q() {
    if (this.a === STATE_DISPOSED)
      return;
    this.A = void 0;
    this.C = void 0;
    this.B = void 0;
    this.t?.();
    this.t = void 0;
    super.q();
  }
  L() {
    if (this.K && this.a !== STATE_DISPOSED) {
      this.t?.();
      try {
        this.t = this.A(this.e, this.C);
      } catch (e) {
        if (!this.f.notify(this, ERROR_BIT, ERROR_BIT))
          throw e;
      } finally {
        this.C = this.e;
        this.K = false;
      }
    }
  }
};

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
      if (!node.b?.length && node.g?.i !== node) {
        node.dispose();
        node = void 0;
      }
    }
    return resolvedValue;
  };
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
function createRoot(init, options) {
  const owner = new Owner(options?.id);
  return compute(owner, !init.length ? init : () => init(() => owner.dispose()), null);
}
function runWithOwner(owner, run) {
  return compute(owner, run, null);
}

export { Owner, createEffect, createMemo, createRoot, createSignal, getOwner, onCleanup, runWithOwner, untrack };
