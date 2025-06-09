// @ts-nocheck

// src/core/error.ts
var NotReadyError = class extends Error {};
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
  if (scheduled) return;
  scheduled = true;
  if (!globalQueue.u) queueMicrotask(flushSync);
}
var pureQueue = [];
var Queue = class {
  i = null;
  u = false;
  v = [[], []];
  t = [];
  created = clock;
  enqueue(type, fn) {
    pureQueue.push(fn);
    if (type) this.v[type - 1].push(fn);
    schedule();
  }
  run(type) {
    if (type === EFFECT_PURE) {
      pureQueue.length && runQueue(pureQueue, type);
      pureQueue = [];
      return;
    } else if (this.v[type - 1].length) {
      const effects = this.v[type - 1];
      this.v[type - 1] = [];
      runQueue(effects, type);
    }
    for (let i = 0; i < this.t.length; i++) {
      this.t[i].run(type);
    }
  }
  flush() {
    if (this.u) return;
    this.u = true;
    try {
      this.run(EFFECT_PURE);
      incrementClock();
      scheduled = false;
      this.run(EFFECT_RENDER);
      this.run(EFFECT_USER);
    } finally {
      this.u = false;
    }
  }
  addChild(child) {
    this.t.push(child);
    child.i = this;
  }
  removeChild(child) {
    const index = this.t.indexOf(child);
    if (index >= 0) this.t.splice(index, 1);
  }
  notify(...args) {
    if (this.i) return this.i.notify(...args);
    return false;
  }
};
var globalQueue = new Queue();
function flushSync() {
  while (scheduled) {
    globalQueue.flush();
  }
}
function runQueue(queue, type) {
  for (let i = 0; i < queue.length; i++) queue[i](type);
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
  i = null;
  h = null;
  l = null;
  a = STATE_CLEAN;
  f = null;
  j = defaultContext;
  g = globalQueue;
  F = 0;
  id = null;
  constructor(id = null, skipAppend = false) {
    this.id = id;
    if (currentOwner) {
      !skipAppend && currentOwner.append(this);
    }
  }
  append(child) {
    child.i = this;
    child.l = this;
    if (this.h) this.h.l = child;
    child.h = this.h;
    this.h = child;
    if (this.id != null && child.id == null) child.id = this.getNextChildId();
    if (child.j !== this.j) {
      child.j = { ...this.j, ...child.j };
    }
    if (this.g) child.g = this.g;
  }
  dispose(self = true) {
    if (this.a === STATE_DISPOSED) return;
    let head = self ? this.l || this.i : this,
      current = this.h,
      next = null;
    while (current && current.i === this) {
      current.dispose(true);
      current.o();
      next = current.h;
      current.h = null;
      current = next;
    }
    this.F = 0;
    if (self) this.o();
    if (current) current.l = !self ? this : this.l;
    if (head) head.h = current;
  }
  o() {
    if (this.l) this.l.h = null;
    this.i = null;
    this.l = null;
    this.j = defaultContext;
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
  getNextChildId() {
    if (this.id != null) return formatId(this.id, this.F++);
    throw new Error("Cannot get child id from owner without an id");
  }
};
function onCleanup(fn) {
  if (!currentOwner) return fn;
  const node = currentOwner;
  if (!node.f) {
    node.f = fn;
  } else if (Array.isArray(node.f)) {
    node.f.push(fn);
  } else {
    node.f = [node.f, fn];
  }
  return fn;
}
function formatId(prefix, id) {
  const num = id.toString(36),
    len = num.length - 1;
  return prefix + (len ? String.fromCharCode(64 + len) : "") + num;
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
  p;
  // Used in __DEV__ mode, hopefully removed in production
  J;
  // Using false is an optimization as an alternative to _equals: () => false
  // which could enable more efficient DIRTY notification
  A = isEqual;
  G;
  /** Whether the computation is an error or has ancestors that are unresolved */
  d = 0;
  /** Which flags raised by sources are handled, vs. being passed through. */
  B = DEFAULT_FLAGS;
  q = -1;
  n = false;
  constructor(initialValue, compute2, options) {
    super(options?.id, compute2 === null);
    this.p = compute2;
    this.a = compute2 ? STATE_DIRTY : STATE_CLEAN;
    this.d = compute2 && initialValue === void 0 ? UNINITIALIZED_BIT : 0;
    this.e = initialValue;
    if (options?.equals !== void 0) this.A = options.equals;
    if (options?.unobserved) this.G = options?.unobserved;
  }
  H() {
    if (this.p) {
      if (this.d & ERROR_BIT && this.q <= getClock()) update(this);
      else this.r();
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
    return this.H();
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   *
   * If the computation has any unresolved ancestors, this function waits for the value to resolve
   * before continuing
   */
  wait() {
    if (this.p && this.d & ERROR_BIT && this.q <= getClock()) {
      update(this);
    } else {
      this.r();
    }
    track(this);
    if ((notStale || this.d & UNINITIALIZED_BIT) && this.d & LOADING_BIT) {
      throw new NotReadyError();
    }
    return this.H();
  }
  /** Update the computation with a new value. */
  write(value, flags = 0, raw = false) {
    const newValue =
      !raw && typeof value === "function" ? value(this.e) : value;
    const valueChanged =
      newValue !== UNCHANGED &&
      (!!(this.d & UNINITIALIZED_BIT) ||
        this.d & LOADING_BIT & ~flags ||
        this.A === false ||
        !this.A(this.e, newValue));
    if (valueChanged) {
      this.e = newValue;
      this.w = void 0;
    }
    const changedFlagsMask = this.d ^ flags,
      changedFlags = changedFlagsMask & flags;
    this.d = flags;
    this.q = getClock() + 1;
    if (this.c) {
      for (let i = 0; i < this.c.length; i++) {
        if (valueChanged) {
          this.c[i].k(STATE_DIRTY);
        } else if (changedFlagsMask) {
          this.c[i].I(changedFlagsMask, changedFlags);
        }
      }
    }
    return this.e;
  }
  /**
   * Set the current node's state, and recursively mark all of this node's observers as STATE_CHECK
   */
  k(state, skipQueue) {
    if (this.a >= state && !this.n) return;
    this.n = !!skipQueue;
    this.a = state;
    if (this.c) {
      for (let i = 0; i < this.c.length; i++) {
        this.c[i].k(STATE_CHECK, skipQueue);
      }
    }
  }
  /**
   * Notify the computation that one of its sources has changed flags.
   *
   * @param mask A bitmask for which flag(s) were changed.
   * @param newFlags The source's new flags, masked to just the changed ones.
   */
  I(mask, newFlags2) {
    if (this.a >= STATE_DIRTY) return;
    if (mask & this.B) {
      this.k(STATE_DIRTY);
      return;
    }
    if (this.a >= STATE_CHECK) return;
    const prevFlags = this.d & mask;
    const deltaFlags = prevFlags ^ newFlags2;
    if (newFlags2 === prevFlags);
    else if (deltaFlags & prevFlags & mask) {
      this.k(STATE_CHECK);
    } else {
      this.d ^= deltaFlags;
      if (this.c) {
        for (let i = 0; i < this.c.length; i++) {
          this.c[i].I(mask, newFlags2);
        }
      }
    }
  }
  C(error) {
    this.w = error;
    this.write(
      UNCHANGED,
      (this.d & ~LOADING_BIT) | ERROR_BIT | UNINITIALIZED_BIT
    );
  }
  /**
   * This is the core part of the reactivity system, which makes sure that the values are updated
   * before they are read. We've also adapted it to return the loading state of the computation,
   * so that we can propagate that to the computation's observers.
   *
   * This function will ensure that the value and states we read from the computation are up to date
   */
  r() {
    if (!this.p) {
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
        this.b[i].r();
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
  o() {
    if (this.a === STATE_DISPOSED) return;
    if (this.b) removeSourceObservers(this, 0);
    super.o();
  }
};
function track(computation) {
  if (currentObserver) {
    if (
      !newSources &&
      currentObserver.b &&
      currentObserver.b[newSourcesIndex] === computation
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
    const result = compute(node, node.p, node);
    node.write(result, newFlags, true);
  } catch (error) {
    if (error instanceof NotReadyError) {
      node.write(
        UNCHANGED,
        newFlags | LOADING_BIT | (node.d & UNINITIALIZED_BIT)
      );
    } else {
      node.C(error);
    }
  } finally {
    if (newSources) {
      if (node.b) removeSourceObservers(node, newSourcesIndex);
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
        if (!source.c) source.c = [node];
        else source.c.push(node);
      }
    } else if (node.b && newSourcesIndex < node.b.length) {
      removeSourceObservers(node, newSourcesIndex);
      node.b.length = newSourcesIndex;
    }
    newSources = prevSources;
    newSourcesIndex = prevSourcesIndex;
    newFlags = prevFlags;
    node.q = getClock() + 1;
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
      if (!source.c.length) source.G?.();
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
function latest(fn, fallback) {
  const argLength = arguments.length;
  const prevFlags = newFlags;
  const prevNotStale = notStale;
  notStale = false;
  try {
    return fn();
  } catch (err) {
    if (argLength > 1 && err instanceof NotReadyError) return fallback;
    throw err;
  } finally {
    newFlags = prevFlags;
    notStale = prevNotStale;
  }
}
function compute(owner, fn, observer) {
  const prevOwner = setOwner(owner),
    prevObserver = currentObserver,
    prevMask = currentMask,
    prevNotStale = notStale;
  currentObserver = observer;
  currentMask = observer?.B ?? DEFAULT_FLAGS;
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
  x;
  y;
  s;
  D = false;
  z;
  m;
  constructor(initialValue, compute2, effect, error, options) {
    super(initialValue, compute2, options);
    this.x = effect;
    this.y = error;
    this.z = initialValue;
    this.m = options?.render ? EFFECT_RENDER : EFFECT_USER;
    if (this.m === EFFECT_RENDER) {
      this.p = (p) =>
        getClock() > this.g.created && !(this.d & ERROR_BIT)
          ? latest(() => compute2(p))
          : compute2(p);
    }
    this.r();
    !options?.defer &&
      (this.m === EFFECT_USER
        ? this.g.enqueue(this.m, this.E.bind(this))
        : this.E(this.m));
  }
  write(value, flags = 0) {
    if (this.a == STATE_DIRTY) {
      this.d;
      this.d = flags;
      if (this.m === EFFECT_RENDER) {
        this.g.notify(this, LOADING_BIT | ERROR_BIT, flags);
      }
    }
    if (value === UNCHANGED) return this.e;
    this.e = value;
    this.D = true;
    return value;
  }
  k(state, skipQueue) {
    if (this.a >= state || skipQueue) return;
    if (this.a === STATE_CLEAN) this.g.enqueue(this.m, this.E.bind(this));
    this.a = state;
  }
  C(error) {
    this.w = error;
    this.s?.();
    this.g.notify(this, LOADING_BIT, 0);
    this.d = ERROR_BIT;
    if (this.m === EFFECT_USER) {
      try {
        return this.y
          ? (this.s = this.y(error))
          : console.error(new EffectError(this.x, error));
      } catch (e) {
        error = e;
      }
    }
    if (!this.g.notify(this, ERROR_BIT, ERROR_BIT)) throw error;
  }
  o() {
    if (this.a === STATE_DISPOSED) return;
    this.x = void 0;
    this.z = void 0;
    this.y = void 0;
    this.s?.();
    this.s = void 0;
    super.o();
  }
  E(type) {
    if (type) {
      if (this.D && this.a !== STATE_DISPOSED) {
        this.s?.();
        try {
          this.s = this.x(this.e, this.z);
        } catch (e) {
          if (!this.g.notify(this, ERROR_BIT, ERROR_BIT)) throw e;
        } finally {
          this.z = this.e;
          this.D = false;
        }
      }
    } else this.a !== STATE_CLEAN && runTop(this);
  }
};
function runTop(node) {
  const ancestors = [];
  for (let current = node; current !== null; current = current.i) {
    if (current.a !== STATE_CLEAN) {
      ancestors.push(current);
    }
  }
  for (let i = ancestors.length - 1; i >= 0; i--) {
    if (ancestors[i].a !== STATE_DISPOSED) ancestors[i].r();
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
  let node = new Computation(value, compute2, options);
  let resolvedValue;
  return () => {
    if (node) {
      if (node.a === STATE_DISPOSED) {
        node = void 0;
        return resolvedValue;
      }
      resolvedValue = node.wait();
      if (!node.b?.length && node.h?.i !== node) {
        node.dispose();
        node = void 0;
      }
    }
    return resolvedValue;
  };
}
function createEffect(compute2, effect, error, value, options) {
  void new Effect(value, compute2, effect, error, options);
}
function createRoot(init, options) {
  const owner = new Owner(options?.id);
  return compute(
    owner,
    !init.length ? init : () => init(() => owner.dispose()),
    null
  );
}
function runWithOwner(owner, run) {
  return compute(owner, run, null);
}

export {
  Owner,
  createEffect,
  createMemo,
  createRoot,
  createSignal,
  getOwner,
  onCleanup,
  runWithOwner,
  untrack,
};
