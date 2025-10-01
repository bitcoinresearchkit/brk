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

// src/core/flags.ts
var ERROR_OFFSET = 0;
var ERROR_BIT = 1 << ERROR_OFFSET;
var LOADING_OFFSET = 1;
var LOADING_BIT = 1 << LOADING_OFFSET;
var UNINITIALIZED_OFFSET = 2;
var UNINITIALIZED_BIT = 1 << UNINITIALIZED_OFFSET;
var DEFAULT_FLAGS = ERROR_BIT;

// src/core/scheduler.ts
var clock = 0;
function incrementClock() {
  clock++;
}
var ActiveTransition = null;
var Unobserved = [];
var scheduled = false;
function schedule() {
  if (scheduled)
    return;
  scheduled = true;
  if (!globalQueue.A)
    queueMicrotask(flush);
}
function notifyUnobserved() {
  for (let i = 0; i < Unobserved.length; i++) {
    const source = Unobserved[i];
    if (!source.b || !source.b.length)
      Unobserved[i].ea?.();
  }
  Unobserved = [];
}
var pureQueue = [];
var Queue = class {
  k = null;
  A = false;
  g = [[], []];
  f = [];
  created = clock;
  enqueue(type, fn) {
    pureQueue.push(fn);
    if (type)
      this.g[type - 1].push(fn);
    schedule();
  }
  run(type) {
    if (type === EFFECT_PURE) {
      pureQueue.length && runQueue(pureQueue, type);
      pureQueue = [];
      return;
    } else if (this.g[type - 1].length) {
      const effects = this.g[type - 1];
      this.g[type - 1] = [];
      runQueue(effects, type);
    }
    for (let i = 0; i < this.f.length; i++) {
      this.f[i].run(type);
    }
  }
  flush() {
    if (this.A)
      return;
    this.A = true;
    try {
      this.run(EFFECT_PURE);
      incrementClock();
      scheduled = false;
      this.run(EFFECT_RENDER);
      this.run(EFFECT_USER);
    } finally {
      this.A = false;
      Unobserved.length && notifyUnobserved();
    }
  }
  addChild(child) {
    if (ActiveTransition && ActiveTransition.r.has(this))
      return ActiveTransition.r.get(this).addChild(child);
    this.f.push(child);
    child.k = this;
  }
  removeChild(child) {
    if (ActiveTransition && ActiveTransition.r.has(this))
      return ActiveTransition.r.get(this).removeChild(child);
    const index = this.f.indexOf(child);
    if (index >= 0) {
      this.f.splice(index, 1);
      child.k = null;
    }
  }
  notify(...args) {
    if (this.k)
      return this.k.notify(...args);
    return false;
  }
  merge(queue) {
    this.g[0].push.apply(this.g[0], queue.g[0]);
    this.g[1].push.apply(this.g[1], queue.g[1]);
    for (let i = 0; i < queue.f.length; i++) {
      const og = this.f.find((c) => c.e === queue.f[i].e);
      if (og)
        og.merge(queue.f[i]);
      else
        this.addChild(queue.f[i]);
    }
  }
};
var globalQueue = new Queue();
function flush() {
  while (scheduled) {
    globalQueue.flush();
  }
}
function removeSourceObservers(node, index) {
  let source;
  let swap;
  for (let i = index; i < node.a.length; i++) {
    source = getTransitionSource(node.a[i]);
    if (source.b) {
      if ((swap = source.b.indexOf(node)) !== -1) {
        source.b[swap] = source.b[source.b.length - 1];
        source.b.pop();
      }
      if (!source.b.length)
        Unobserved.push(source);
    }
  }
}
function runQueue(queue, type) {
  for (let i = 0; i < queue.length; i++)
    queue[i](type);
}
var Transition = class _Transition {
  a = /* @__PURE__ */ new Map();
  s = /* @__PURE__ */ new Set();
  H = /* @__PURE__ */ new Set();
  W = /* @__PURE__ */ new Set();
  u = false;
  g = [[], []];
  r = /* @__PURE__ */ new Map();
  I = [];
  f = [];
  k = null;
  A = false;
  X = false;
  e = globalQueue;
  created = clock;
  constructor() {
    this.r.set(globalQueue, this);
    for (const child of globalQueue.f) {
      cloneQueue(child, this, this.r);
    }
  }
  enqueue(type, fn) {
    this.I.push(fn);
    if (type)
      this.g[type - 1].push(fn);
    this.schedule();
  }
  run(type) {
    if (type === EFFECT_PURE) {
      this.I.length && runQueue(this.I, type);
      this.I = [];
      return;
    } else if (this.g[type - 1].length) {
      const effects = this.g[type - 1];
      this.g[type - 1] = [];
      runQueue(effects, type);
    }
    for (let i = 0; i < this.f.length; i++) {
      this.f[i].run(type);
    }
  }
  flush() {
    if (this.A)
      return;
    this.A = true;
    let currentTransition = ActiveTransition;
    ActiveTransition = this;
    try {
      this.run(EFFECT_PURE);
      incrementClock();
      this.X = false;
      ActiveTransition = currentTransition;
      finishTransition(this);
    } finally {
      this.A = false;
      ActiveTransition = currentTransition;
    }
  }
  addChild(child) {
    this.f.push(child);
    child.k = this;
  }
  removeChild(child) {
    const index = this.f.indexOf(child);
    if (index >= 0)
      this.f.splice(index, 1);
  }
  notify(node, type, flags) {
    if (!(type & LOADING_BIT))
      return false;
    if (flags & LOADING_BIT) {
      this.s.add(node);
    } else {
      this.s.delete(node);
    }
    return true;
  }
  merge(queue) {
    this.g[0].push.apply(this.g[0], queue.g[0]);
    this.g[1].push.apply(this.g[1], queue.g[1]);
    this.I.push.apply(this.I, queue.I);
    for (let i = 0; i < queue.f.length; i++) {
      const og = this.f.find((c) => c.e === queue.f[i].e);
      if (og)
        og.merge(queue.f[i]);
      else
        this.addChild(queue.f[i]);
    }
  }
  schedule() {
    if (this.X)
      return;
    this.X = true;
    if (!this.A)
      queueMicrotask(() => this.flush());
  }
  runTransition(fn, force = false) {
    if (this.u) {
      if (this.u instanceof _Transition)
        return this.u.runTransition(fn, force);
      if (!force)
        throw new Error("Transition already completed");
      fn();
      return;
    }
    ActiveTransition = this;
    try {
      let result = fn();
      let transition2 = ActiveTransition;
      if (result?.next) {
        (async function() {
          let temp, value;
          while (!(temp = result.next(value)).done) {
            if (temp.value instanceof Promise) {
              transition2.H.add(temp.value);
              try {
                value = await temp.value;
              } finally {
                while (transition2.u instanceof _Transition)
                  transition2 = transition2.u;
                transition2.H.delete(temp.value);
              }
              ActiveTransition = transition2;
            } else
              value = temp.value;
          }
          ActiveTransition = null;
          finishTransition(transition2);
        })();
      }
      if (result instanceof Promise) {
        transition2.H.add(result);
        result.finally(() => {
          while (transition2.u instanceof _Transition)
            transition2 = transition2.u;
          transition2.H.delete(result);
          ActiveTransition = null;
          finishTransition(transition2);
        });
      }
    } finally {
      const transition2 = ActiveTransition;
      ActiveTransition = null;
      finishTransition(transition2);
    }
  }
  addOptimistic(fn) {
    if (fn.j && fn.j !== this) {
      mergeTransitions(fn.j, this);
      ActiveTransition = fn.j;
      return;
    }
    fn.j = this;
    this.W.add(fn);
  }
};
function transition(fn) {
  let t = new Transition();
  queueMicrotask(() => t.runTransition(() => fn((fn2) => t.runTransition(fn2))));
}
function cloneGraph(node) {
  if (node.j) {
    if (node.j !== ActiveTransition) {
      mergeTransitions(node.j, ActiveTransition);
      ActiveTransition = node.j;
    }
    return node.j.a.get(node);
  }
  const clone = Object.create(Object.getPrototypeOf(node));
  Object.assign(clone, node, {
    n: null,
    m: null,
    b: null,
    a: node.a ? [...node.a] : null,
    e: node
  });
  ActiveTransition.a.set(node, clone);
  node.j = ActiveTransition;
  if (node.a) {
    for (let i = 0; i < node.a.length; i++)
      node.a[i].b.push(clone);
  }
  if (node.b) {
    clone.b = [];
    for (let i = 0, length = node.b.length; i < length; i++) {
      !node.b[i].e && clone.b.push(cloneGraph(node.b[i]));
    }
  }
  return clone;
}
function replaceSourceObservers(node, transition2) {
  let source;
  let swap;
  for (let i = 0; i < node.a.length; i++) {
    source = transition2.a.get(node.a[i]) || node.a[i];
    if (source.b && (swap = source.b.indexOf(node)) !== -1) {
      const remove = source.b.indexOf(node.e) > -1;
      source.b[swap] = !remove ? node.e : source.b[source.b.length - 1];
      remove && source.b.pop();
    }
  }
}
function cloneQueue(queue, parent, clonedQueues) {
  const clone = Object.create(Object.getPrototypeOf(queue));
  Object.assign(clone, queue, {
    e: queue,
    k: parent,
    f: [],
    enqueue(type, fn) {
      ActiveTransition?.enqueue(type, fn);
    },
    notify(node, type, flags) {
      node = node.e || node;
      if (!clone.J || type & LOADING_BIT) {
        type &= ~LOADING_BIT;
        ActiveTransition?.notify(node, LOADING_BIT, flags);
        if (!type)
          return true;
      }
      return queue.notify.call(this, node, type, flags);
    }
  });
  parent.f.push(clone);
  clonedQueues.set(queue, clone);
  for (const child of queue.f) {
    cloneQueue(child, clone, clonedQueues);
  }
}
function resolveQueues(children) {
  for (const child of children) {
    const og = child.e;
    if (og) {
      const clonedChildren = child.f;
      delete child.enqueue;
      delete child.notify;
      delete child.k;
      delete child.f;
      Object.assign(og, child);
      delete og.e;
      resolveQueues(clonedChildren);
    } else if (child.k.e) {
      child.k.e.addChild(child);
    }
  }
}
function mergeTransitions(t1, t2) {
  t2.a.forEach((value, key) => {
    key.j = t1;
    t1.a.set(key, value);
  });
  t2.W.forEach((c) => {
    c.j = t1;
    t1.W.add(c);
  });
  t2.H.forEach((p) => t1.H.add(p));
  t2.s.forEach((n) => t1.s.add(n));
  t1.merge(t2);
  t2.u = t1;
}
function getTransitionSource(input) {
  return ActiveTransition && ActiveTransition.a.get(input) || input;
}
function getQueue(node) {
  const transition2 = ActiveTransition || node.e?.j;
  return transition2 && transition2.r.get(node.B) || node.B;
}
function initialDispose(node) {
  let current = node.m;
  while (current !== null && current.k === node) {
    initialDispose(current);
    const clone = ActiveTransition.a.get(current);
    if (clone && !clone.Y)
      clone.dispose(true);
    current = current.m;
  }
}
function finishTransition(transition2) {
  if (transition2.u || transition2.X || transition2.H.size || transition2.s.size)
    return;
  globalQueue.g[0].push.apply(globalQueue.g[0], transition2.g[0]);
  globalQueue.g[1].push.apply(globalQueue.g[1], transition2.g[1]);
  resolveQueues(transition2.f);
  for (const [source, clone] of transition2.a) {
    if (source === clone || source.j !== transition2) {
      delete source.j;
      continue;
    }
    if (clone.a)
      replaceSourceObservers(clone, transition2);
    if (clone.Y || clone.c === STATE_DISPOSED) {
      source.dispose(clone.c === STATE_DISPOSED);
      source.emptyDisposal();
      delete clone.Y;
    } else {
      delete clone.m;
      delete clone.n;
    }
    Object.assign(source, clone);
    delete source.e;
    let current = clone.m;
    if (current?.w === clone)
      current.w = source;
    while (current?.k === clone) {
      current.k = source;
      current = current.m;
    }
    delete source.j;
  }
  transition2.u = true;
  for (const reset of transition2.W) {
    delete reset.j;
    reset();
  }
  globalQueue.flush();
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
  k = null;
  m = null;
  w = null;
  c = STATE_CLEAN;
  n = null;
  x = defaultContext;
  B = globalQueue;
  fa = 0;
  id = null;
  constructor(id = null, skipAppend = false) {
    this.id = id;
    if (currentOwner) {
      !skipAppend && currentOwner.append(this);
    }
  }
  append(child) {
    child.k = this;
    child.w = this;
    if (this.m)
      this.m.w = child;
    child.m = this.m;
    this.m = child;
    if (this.id != null && child.id == null)
      child.id = this.getNextChildId();
    if (child.x !== this.x) {
      child.x = { ...this.x, ...child.x };
    }
    if (this.B)
      child.B = this.B;
  }
  dispose(self = true) {
    if (this.c === STATE_DISPOSED)
      return;
    let head = self ? this.w || this.k : this, current = this.m, next = null;
    while (current && current.k === this) {
      current.dispose(true);
      next = current.m;
      current.m = null;
      current = next;
    }
    this.fa = 0;
    if (self)
      this.R();
    if (current)
      current.w = !self ? this : this.w;
    if (head)
      head.m = current;
  }
  R() {
    if (this.w)
      this.w.m = null;
    this.k = null;
    this.w = null;
    this.x = defaultContext;
    this.c = STATE_DISPOSED;
    this.emptyDisposal();
  }
  emptyDisposal() {
    if (!this.n)
      return;
    if (Array.isArray(this.n)) {
      for (let i = 0; i < this.n.length; i++) {
        const callable = this.n[i];
        callable.call(callable);
      }
    } else {
      this.n.call(this.n);
    }
    this.n = null;
  }
  getNextChildId() {
    if (this.id != null)
      return formatId(this.id, this.fa++);
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
  const value = hasContext(context, owner) ? owner.x[context.id] : context.defaultValue;
  if (isUndefined(value)) {
    throw new ContextNotFoundError();
  }
  return value;
}
function setContext(context, value, owner = currentOwner) {
  if (!owner) {
    throw new NoOwnerError();
  }
  owner.x = {
    ...owner.x,
    [context.id]: isUndefined(value) ? context.defaultValue : value
  };
}
function hasContext(context, owner = currentOwner) {
  return !isUndefined(owner?.x[context.id]);
}
function onCleanup(fn) {
  if (!currentOwner)
    return fn;
  const node = currentOwner;
  if (!node.n) {
    node.n = fn;
  } else if (Array.isArray(node.n)) {
    node.n.push(fn);
  } else {
    node.n = [node.n, fn];
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
  a = null;
  b = null;
  l;
  L;
  M;
  // Used in __DEV__ mode, hopefully removed in production
  la;
  // Using false is an optimization as an alternative to _equals: () => false
  // which could enable more efficient DIRTY notification
  aa = isEqual;
  ea;
  ha = false;
  /** Whether the computation is an error or has ancestors that are unresolved */
  h = 0;
  /** Which flags raised by sources are handled, vs. being passed through. */
  ba = DEFAULT_FLAGS;
  N = -1;
  D = false;
  j;
  e;
  constructor(initialValue, compute2, options) {
    super(options?.id, compute2 === null);
    this.M = compute2;
    this.c = compute2 ? STATE_DIRTY : STATE_CLEAN;
    this.h = compute2 && initialValue === void 0 ? UNINITIALIZED_BIT : 0;
    this.l = initialValue;
    if (options?.equals !== void 0)
      this.aa = options.equals;
    if (options?.pureWrite)
      this.ha = true;
    if (options?.unobserved)
      this.ea = options?.unobserved;
    if (ActiveTransition) {
      this.j = ActiveTransition;
      ActiveTransition.a.set(this, this);
    }
  }
  ga() {
    track(this);
    newFlags |= this.h & ~currentMask;
    if (this.h & ERROR_BIT) {
      throw this.L;
    } else {
      return this.l;
    }
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   */
  read() {
    if (ActiveTransition && (ActiveTransition.a.has(this) || !this.e && this.h & (UNINITIALIZED_BIT | ERROR_BIT))) {
      const clone = ActiveTransition.a.get(this) || cloneGraph(this);
      if (clone !== this)
        return clone.read();
    }
    if (this.M) {
      if (this.h & ERROR_BIT && this.N <= clock)
        update(this);
      else
        this.K();
    }
    return this.ga();
  }
  /**
   * Return the current value of this computation
   * Automatically re-executes the surrounding computation when the value changes
   *
   * If the computation has any unresolved ancestors, this function waits for the value to resolve
   * before continuing
   */
  wait() {
    if (ActiveTransition && (ActiveTransition.a.has(this) || !this.e && this.h & (UNINITIALIZED_BIT | ERROR_BIT))) {
      const clone = ActiveTransition.a.get(this) || cloneGraph(this);
      if (clone !== this)
        return clone.wait();
    }
    if (this.M) {
      if (this.h & ERROR_BIT && this.N <= clock)
        update(this);
      else
        this.K();
    }
    if ((notStale || this.h & UNINITIALIZED_BIT) && this.h & LOADING_BIT) {
      track(this);
      throw new NotReadyError();
    }
    if (staleCheck && this.h & LOADING_BIT) {
      staleCheck.l = true;
    }
    return this.ga();
  }
  /** Update the computation with a new value. */
  write(value, flags = 0, raw = false) {
    if (ActiveTransition && !this.e) {
      const clone = cloneGraph(this);
      if (clone !== this)
        return clone.write(value, flags, raw);
    }
    const newValue = !raw && typeof value === "function" ? value(this.l) : value;
    const valueChanged = newValue !== UNCHANGED && (!!(this.h & UNINITIALIZED_BIT) || // this._stateFlags & LOADING_BIT & ~flags ||
    this.aa === false || !this.aa(this.l, newValue));
    if (valueChanged) {
      this.l = newValue;
      this.L = void 0;
    }
    const changedFlagsMask = this.h ^ flags, changedFlags = changedFlagsMask & flags;
    this.h = flags;
    this.N = clock + 1;
    if (this.b) {
      for (let i = 0; i < this.b.length; i++) {
        if (valueChanged) {
          this.b[i].y(STATE_DIRTY);
        } else if (changedFlagsMask) {
          this.b[i].Z(changedFlagsMask, changedFlags);
        }
      }
    }
    return this.l;
  }
  /**
   * Set the current node's state, and recursively mark all of this node's observers as STATE_CHECK
   */
  y(state, skipQueue) {
    if (this.c >= state && !this.D)
      return;
    this.D = !!skipQueue;
    this.c = state;
    if (this.b) {
      for (let i = 0; i < this.b.length; i++) {
        this.b[i].y(STATE_CHECK, skipQueue);
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
    if (this.c >= STATE_DIRTY)
      return;
    if (mask & this.ba) {
      this.y(STATE_DIRTY);
      return;
    }
    if (this.c >= STATE_CHECK && !this.D)
      return;
    const prevFlags = this.h & mask;
    const deltaFlags = prevFlags ^ newFlags2;
    if (newFlags2 === prevFlags) ; else if (deltaFlags & prevFlags & mask) {
      this.y(STATE_CHECK);
    } else {
      this.h ^= deltaFlags;
      if (this.b) {
        for (let i = 0; i < this.b.length; i++) {
          this.b[i].Z(mask, newFlags2);
        }
      }
    }
  }
  E(error) {
    if (ActiveTransition && !this.e) {
      const clone = cloneGraph(this);
      if (clone !== this)
        return clone.E(error);
    }
    this.L = error;
    this.write(UNCHANGED, this.h & ~LOADING_BIT | ERROR_BIT | UNINITIALIZED_BIT);
  }
  /**
   * This is the core part of the reactivity system, which makes sure that the values are updated
   * before they are read. We've also adapted it to return the loading state of the computation,
   * so that we can propagate that to the computation's observers.
   *
   * This function will ensure that the value and states we read from the computation are up to date
   */
  K() {
    if (!this.M) {
      return;
    }
    if (this.c === STATE_DISPOSED) {
      return;
    }
    if (this.c === STATE_CLEAN) {
      return;
    }
    let observerFlags = 0;
    if (this.c === STATE_CHECK) {
      for (let i = 0; i < this.a.length; i++) {
        const source = getTransitionSource(this.a[i]);
        source.K();
        observerFlags |= source.h & ~UNINITIALIZED_BIT;
        if (this.c === STATE_DIRTY) {
          break;
        }
      }
    }
    if (this.c === STATE_DIRTY) {
      update(this);
    } else {
      this.write(UNCHANGED, observerFlags);
      this.c = STATE_CLEAN;
    }
  }
  /**
   * Remove ourselves from the owner graph and the computation graph
   */
  R() {
    if (this.c === STATE_DISPOSED)
      return;
    if (this.a)
      removeSourceObservers(this, 0);
    super.R();
  }
};
function track(computation) {
  if (ActiveTransition && computation.e)
    computation = computation.e;
  if (currentObserver) {
    if (!newSources && currentObserver.a && currentObserver.a[newSourcesIndex] === computation) {
      newSourcesIndex++;
    } else if (!newSources)
      newSources = [computation];
    else if (computation !== newSources[newSources.length - 1]) {
      newSources.push(computation);
    }
    if (updateCheck) {
      updateCheck.l = computation.N > currentObserver.N;
    }
  }
}
function update(node) {
  const prevSources = newSources, prevSourcesIndex = newSourcesIndex, prevFlags = newFlags;
  newSources = null;
  newSourcesIndex = 0;
  newFlags = 0;
  try {
    if (ActiveTransition && node.e && !node.Y) {
      initialDispose(node.e);
      node.Y = true;
    }
    node.dispose(false);
    node.emptyDisposal();
    const result = compute(node, node.M, node);
    node.write(result, newFlags, true);
  } catch (error) {
    if (error instanceof NotReadyError) {
      node.write(UNCHANGED, newFlags | LOADING_BIT | node.h & UNINITIALIZED_BIT);
    } else {
      node.E(error);
    }
  } finally {
    if (newSources) {
      if (node.a)
        removeSourceObservers(node, newSourcesIndex);
      if (node.a && newSourcesIndex > 0) {
        node.a.length = newSourcesIndex + newSources.length;
        for (let i = 0; i < newSources.length; i++) {
          node.a[newSourcesIndex + i] = newSources[i];
        }
      } else {
        node.a = newSources;
      }
      let source;
      for (let i = newSourcesIndex; i < node.a.length; i++) {
        source = getTransitionSource(node.a[i]);
        if (!source.b)
          source.b = [node];
        else
          source.b.push(node);
      }
    } else if (node.a && newSourcesIndex < node.a.length) {
      removeSourceObservers(node, newSourcesIndex);
      node.a.length = newSourcesIndex;
    }
    newSources = prevSources;
    newSourcesIndex = prevSourcesIndex;
    newFlags = prevFlags;
    node.N = clock + 1;
    node.c = STATE_CLEAN;
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
  updateCheck = { l: false };
  try {
    fn();
    return updateCheck.l;
  } finally {
    updateCheck = current;
  }
}
function pendingCheck(fn, loadingValue) {
  const current = staleCheck;
  staleCheck = { l: false };
  try {
    latest(fn);
    return staleCheck.l;
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
  c.ba |= LOADING_BIT;
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
  newSourcesIndex = observer.a ? observer.a.length : 0;
  newFlags = 0;
  try {
    return compute(observer, run, observer);
  } catch (error) {
    if (error instanceof NotReadyError) {
      observer.write(
        UNCHANGED,
        newFlags | LOADING_BIT | observer.h & UNINITIALIZED_BIT
      );
    } else {
      observer.E(error);
    }
  } finally {
    if (newSources) {
      if (newSourcesIndex > 0) {
        observer.a.length = newSourcesIndex + newSources.length;
        for (let i = 0; i < newSources.length; i++) {
          observer.a[newSourcesIndex + i] = newSources[i];
        }
      } else {
        observer.a = newSources;
      }
      let source;
      for (let i = newSourcesIndex; i < observer.a.length; i++) {
        source = observer.a[i];
        if (!source.b)
          source.b = [observer];
        else
          source.b.push(observer);
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
  currentMask = observer?.ba ?? DEFAULT_FLAGS;
  notStale = true;
  try {
    return fn.call(observer, observer ? observer.l : void 0);
  } finally {
    setOwner(prevOwner);
    currentObserver = prevObserver;
    currentMask = prevMask;
    notStale = prevNotStale;
  }
}

// src/core/effect.ts
var Effect = class extends Computation {
  ca;
  _;
  O;
  da = false;
  $;
  C;
  constructor(initialValue, compute2, effect, error, options) {
    super(initialValue, compute2, options);
    this.ca = effect;
    this._ = error;
    this.$ = initialValue;
    this.C = options?.render ? EFFECT_RENDER : EFFECT_USER;
    if (this.C === EFFECT_RENDER) {
      this.M = function(p) {
        return !this.e && clock > this.B.created && !(this.h & ERROR_BIT) ? latest(() => compute2(p)) : compute2(p);
      };
    }
    this.K();
    !options?.defer && (this.C === EFFECT_USER ? getQueue(this).enqueue(this.C, this.F.bind(this)) : this.F(this.C));
  }
  write(value, flags = 0) {
    if (this.c == STATE_DIRTY) {
      this.h = flags;
      if (this.C === EFFECT_RENDER) {
        getQueue(this).notify(this, LOADING_BIT | ERROR_BIT, this.h);
      }
    }
    if (value === UNCHANGED)
      return this.l;
    this.l = value;
    this.da = true;
    this.L = void 0;
    return value;
  }
  y(state, skipQueue) {
    if (this.c >= state || skipQueue)
      return;
    if (this.c === STATE_CLEAN)
      getQueue(this).enqueue(this.C, this.F.bind(this));
    this.c = state;
  }
  Z(mask, newFlags2) {
    if (this.e) {
      if (this.c >= STATE_DIRTY)
        return;
      if (mask & 3) {
        this.y(STATE_DIRTY);
        return;
      }
    }
    super.Z(mask, newFlags2);
  }
  E(error) {
    this.L = error;
    getQueue(this).notify(this, LOADING_BIT, 0);
    this.h = ERROR_BIT;
    if (this.C === EFFECT_USER) {
      try {
        return this._ ? this._(error, () => {
          this.O?.();
          this.O = void 0;
        }) : console.error(error);
      } catch (e) {
        error = e;
      }
    }
    if (!getQueue(this).notify(this, ERROR_BIT, ERROR_BIT))
      throw error;
  }
  R() {
    if (this.c === STATE_DISPOSED)
      return;
    this.ca = void 0;
    this.$ = void 0;
    this._ = void 0;
    this.O?.();
    this.O = void 0;
    getQueue(this).notify(this, ERROR_BIT | LOADING_BIT, 0);
    super.R();
  }
  F(type) {
    if (type) {
      const effect = this.e || this;
      if (effect.da && effect.c !== STATE_DISPOSED) {
        effect.O?.();
        try {
          effect.O = effect.ca(effect.l, effect.$);
        } catch (e) {
          if (!getQueue(effect).notify(effect, ERROR_BIT, ERROR_BIT))
            throw e;
        } finally {
          effect.$ = effect.l;
          effect.da = false;
        }
      }
    } else
      this.c !== STATE_CLEAN && runTop(this);
  }
};
var EagerComputation = class extends Computation {
  constructor(initialValue, compute2, options) {
    super(initialValue, compute2, options);
    !options?.defer && this.K();
  }
  y(state, skipQueue) {
    if (this.c >= state && !this.D)
      return;
    if (!skipQueue && (this.c === STATE_CLEAN || this.c === STATE_CHECK && this.D))
      getQueue(this).enqueue(EFFECT_PURE, this.F.bind(this));
    super.y(state, skipQueue);
  }
  F() {
    this.c !== STATE_CLEAN && runTop(this);
  }
};
var FirewallComputation = class extends Computation {
  firewall = true;
  constructor(compute2) {
    super(void 0, compute2);
  }
  y(state, skipQueue) {
    if (this.c >= state && !this.D)
      return;
    if (!skipQueue && (this.c === STATE_CLEAN || this.c === STATE_CHECK && this.D))
      getQueue(this).enqueue(EFFECT_PURE, this.F.bind(this));
    super.y(state, true);
    this.D = !!skipQueue;
  }
  F() {
    this.c !== STATE_CLEAN && runTop(this);
  }
};
function runTop(node) {
  const ancestors = [];
  for (let current = node; current !== null; current = current.k) {
    if (ActiveTransition && current.j)
      current = ActiveTransition.a.get(current);
    if (current.c !== STATE_CLEAN) {
      ancestors.push(current);
    }
  }
  for (let i = ancestors.length - 1; i >= 0; i--) {
    if (ancestors[i].c !== STATE_DISPOSED)
      ancestors[i].K();
  }
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
    if (state == null)
      throw new Error("Cannot reconcile null or undefined state");
    const keyFn = typeof key === "string" ? (item) => item[key] : key;
    const eq = keyFn(state);
    if (eq !== void 0 && keyFn(value) !== keyFn(state))
      throw new Error("Cannot reconcile states with different identity");
    applyState(value, state, keyFn, all);
  };
}

// src/store/projection.ts
function createProjection(fn, initialValue = {}, options) {
  let wrappedStore;
  const node = new FirewallComputation(() => {
    storeSetter(wrappedStore, (s) => {
      const value = fn(s);
      if (value !== s && value !== void 0) {
        reconcile(value, options?.key || "id", options?.all)(s);
      }
    });
  });
  const wrappedMap = /* @__PURE__ */ new WeakMap();
  const traps = {
    ...storeTraps,
    get(target, property, receiver) {
      const o = getOwner();
      const n = getTransitionSource(node);
      (!o || o !== n) && n.wait();
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
      let value2 = tracked && (overridden || !proxySource) ? tracked.l : storeValue[property];
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
    const value = fn(store);
    if (value !== store && value !== void 0) {
      if (Array.isArray(value)) {
        for (let i = 0, len = value.length; i < len; i++)
          store[i] = value[i];
        store.length = value.length;
      } else {
        const keys = /* @__PURE__ */ new Set([...Object.keys(store), ...Object.keys(value)]);
        keys.forEach((key) => {
          if (key in value)
            store[key] = value[key];
          else
            delete store[key];
        });
      }
    }
  } finally {
    Writing.clear();
    Writing = prevWriting;
  }
}
function createStore(first, second, options) {
  const derived = typeof first === "function", wrappedStore = derived ? createProjection(first, second, options) : wrap(first);
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
      if (node.c === STATE_DISPOSED) {
        node = void 0;
        return resolvedValue;
      }
      resolvedValue = node.wait();
      if (!node.a?.length && node.m?.k !== node && !(node.h & UNINITIALIZED_BIT)) {
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
      let transition2 = ActiveTransition;
      if (isPromise) {
        source.then(
          (value3) => {
            if (abort)
              return;
            if (transition2)
              return transition2.runTransition(() => {
                node.write(value3, 0, true);
              }, true);
            node.write(value3, 0, true);
          },
          (error) => {
            if (abort)
              return;
            if (transition2)
              return transition2.runTransition(() => node.E(error), true);
            node.E(error);
          }
        );
      } else {
        (async () => {
          try {
            for await (let value3 of source) {
              if (abort)
                return;
              if (transition2)
                return transition2.runTransition(() => {
                  node.write(value3, 0, true);
                  transition2 = null;
                }, true);
              node.write(value3, 0, true);
            }
          } catch (error) {
            if (abort)
              return;
            if (transition2)
              return transition2.runTransition(() => {
                node.E(error);
                transition2 = null;
              }, true);
            node.E(error);
          }
        })();
      }
      throw new NotReadyError();
    },
    options
  );
  const read = node.wait.bind(node);
  read.refresh = () => {
    let n = node;
    if (ActiveTransition && !node.e) {
      n = cloneGraph(node);
    }
    n.c = STATE_DIRTY;
    refreshing = true;
    n.K();
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
function createPending() {
  const node = new Computation(false, null);
  const reset = () => node.write(false);
  function write() {
    if (!ActiveTransition)
      return false;
    ActiveTransition.addOptimistic(reset);
    queueMicrotask(() => reset.j && node.write(true));
  }
  function read() {
    const v = node.read();
    return ActiveTransition ? false : v;
  }
  return [read, write];
}
function useTransition() {
  const [pending, setPending] = createPending();
  function start(fn) {
    transition((resume) => {
      setPending(true);
      return fn(resume);
    });
  }
  return [pending, start];
}
function createOptimistic(first, second, options) {
  let store, setStore;
  if (typeof first === "function") {
    [store, setStore] = createStore((s) => {
      const value = first(s);
      if (!ActiveTransition)
        return value;
      ActiveTransition.addOptimistic(reset);
    }, {});
  } else
    [store, setStore] = createStore(first);
  const reset = () => setStore(
    reconcile(
      typeof first === "function" ? first(second) : first,
      options?.key || "id",
      options?.all
    )
  );
  function write(v) {
    if (!ActiveTransition)
      throw new Error("createOptimistic can only be updated inside a transition");
    ActiveTransition.addOptimistic(reset);
    queueMicrotask(() => reset.j && setStore(v));
  }
  return [store, write];
}

// src/map.ts
function mapArray(list, map, options) {
  const keyFn = typeof options?.keyed === "function" ? options.keyed : void 0;
  return updateKeyedMap.bind({
    S: new Owner(),
    o: 0,
    ia: list,
    G: [],
    P: map,
    i: [],
    d: [],
    Q: keyFn,
    p: keyFn || options?.keyed === false ? [] : void 0,
    q: map.length > 1 ? [] : void 0,
    T: options?.fallback
  });
}
var pureOptions = { pureWrite: true };
function updateKeyedMap() {
  const newItems = this.ia() || [], newLen = newItems.length;
  newItems[$TRACK];
  runWithOwner(this.S, () => {
    let i, j, mapper = this.p ? () => {
      this.p[j] = new Computation(newItems[j], null, pureOptions);
      this.q && (this.q[j] = new Computation(j, null, pureOptions));
      return this.P(
        Computation.prototype.read.bind(this.p[j]),
        this.q ? Computation.prototype.read.bind(this.q[j]) : void 0
      );
    } : this.q ? () => {
      const item = newItems[j];
      this.q[j] = new Computation(j, null, pureOptions);
      return this.P(() => item, Computation.prototype.read.bind(this.q[j]));
    } : () => {
      const item = newItems[j];
      return this.P(() => item);
    };
    if (newLen === 0) {
      if (this.o !== 0) {
        this.S.dispose(false);
        this.d = [];
        this.G = [];
        this.i = [];
        this.o = 0;
        this.p && (this.p = []);
        this.q && (this.q = []);
      }
      if (this.T && !this.i[0]) {
        this.i[0] = compute(
          this.d[0] = new Owner(),
          this.T,
          null
        );
      }
    } else if (this.o === 0) {
      if (this.d[0])
        this.d[0].dispose();
      this.i = new Array(newLen);
      for (j = 0; j < newLen; j++) {
        this.G[j] = newItems[j];
        this.i[j] = compute(this.d[j] = new Owner(), mapper, null);
      }
      this.o = newLen;
    } else {
      let start, end, newEnd, item, key, newIndices, newIndicesNext, temp = new Array(newLen), tempNodes = new Array(newLen), tempRows = this.p ? new Array(newLen) : void 0, tempIndexes = this.q ? new Array(newLen) : void 0;
      for (start = 0, end = Math.min(this.o, newLen); start < end && (this.G[start] === newItems[start] || this.p && compare(this.Q, this.G[start], newItems[start])); start++) {
        if (this.p)
          this.p[start].write(newItems[start]);
      }
      for (end = this.o - 1, newEnd = newLen - 1; end >= start && newEnd >= start && (this.G[end] === newItems[newEnd] || this.p && compare(this.Q, this.G[end], newItems[newEnd])); end--, newEnd--) {
        temp[newEnd] = this.i[end];
        tempNodes[newEnd] = this.d[end];
        tempRows && (tempRows[newEnd] = this.p[end]);
        tempIndexes && (tempIndexes[newEnd] = this.q[end]);
      }
      newIndices = /* @__PURE__ */ new Map();
      newIndicesNext = new Array(newEnd + 1);
      for (j = newEnd; j >= start; j--) {
        item = newItems[j];
        key = this.Q ? this.Q(item) : item;
        i = newIndices.get(key);
        newIndicesNext[j] = i === void 0 ? -1 : i;
        newIndices.set(key, j);
      }
      for (i = start; i <= end; i++) {
        item = this.G[i];
        key = this.Q ? this.Q(item) : item;
        j = newIndices.get(key);
        if (j !== void 0 && j !== -1) {
          temp[j] = this.i[i];
          tempNodes[j] = this.d[i];
          tempRows && (tempRows[j] = this.p[i]);
          tempIndexes && (tempIndexes[j] = this.q[i]);
          j = newIndicesNext[j];
          newIndices.set(key, j);
        } else
          this.d[i].dispose();
      }
      for (j = start; j < newLen; j++) {
        if (j in temp) {
          this.i[j] = temp[j];
          this.d[j] = tempNodes[j];
          if (tempRows) {
            this.p[j] = tempRows[j];
            this.p[j].write(newItems[j]);
          }
          if (tempIndexes) {
            this.q[j] = tempIndexes[j];
            this.q[j].write(j);
          }
        } else {
          this.i[j] = compute(this.d[j] = new Owner(), mapper, null);
        }
      }
      this.i = this.i.slice(0, this.o = newLen);
      this.G = newItems.slice(0);
    }
  });
  return this.i;
}
function repeat(count, map, options) {
  return updateRepeat.bind({
    S: new Owner(),
    o: 0,
    z: 0,
    ja: count,
    P: map,
    d: [],
    i: [],
    ka: options?.from,
    T: options?.fallback
  });
}
function updateRepeat() {
  const newLen = this.ja();
  const from = this.ka?.() || 0;
  runWithOwner(this.S, () => {
    if (newLen === 0) {
      if (this.o !== 0) {
        this.S.dispose(false);
        this.d = [];
        this.i = [];
        this.o = 0;
      }
      if (this.T && !this.i[0]) {
        this.i[0] = compute(
          this.d[0] = new Owner(),
          this.T,
          null
        );
      }
      return;
    }
    const to = from + newLen;
    const prevTo = this.z + this.o;
    if (this.o === 0 && this.d[0])
      this.d[0].dispose();
    for (let i = to; i < prevTo; i++)
      this.d[i - this.z].dispose();
    if (this.z < from) {
      let i = this.z;
      while (i < from && i < this.o)
        this.d[i++].dispose();
      this.d.splice(0, from - this.z);
      this.i.splice(0, from - this.z);
    } else if (this.z > from) {
      let i = prevTo - this.z - 1;
      let difference = this.z - from;
      this.d.length = this.i.length = newLen;
      while (i >= difference) {
        this.d[i] = this.d[i - difference];
        this.i[i] = this.i[i - difference];
        i--;
      }
      for (let i2 = 0; i2 < difference; i2++) {
        this.i[i2] = compute(
          this.d[i2] = new Owner(),
          () => this.P(i2 + from),
          null
        );
      }
    }
    for (let i = prevTo; i < to; i++) {
      this.i[i - from] = compute(
        this.d[i - from] = new Owner(),
        () => this.P(i),
        null
      );
    }
    this.i = this.i.slice(0, newLen);
    this.z = from;
    this.o = newLen;
  });
  return this.i;
}
function compare(key, a, b) {
  return key ? key(a) === key(b) : true;
}

// src/boundaries.ts
var BoundaryComputation = class extends EagerComputation {
  U;
  constructor(compute2, propagationMask) {
    super(void 0, compute2, { defer: true });
    this.U = propagationMask;
  }
  write(value, flags) {
    super.write(value, flags & ~this.U);
    if (this.U & LOADING_BIT && !(this.h & UNINITIALIZED_BIT || ActiveTransition)) {
      flags &= ~LOADING_BIT;
    }
    getQueue(this).notify(this, this.U, flags);
    return this.l;
  }
};
function createBoundChildren(owner, fn, queue, mask) {
  const parentQueue = owner.B;
  parentQueue.addChild(owner.B = queue);
  onCleanup(() => parentQueue.removeChild(owner.B));
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
  t;
  V = /* @__PURE__ */ new Set();
  s = /* @__PURE__ */ new Set();
  constructor(disabled) {
    super();
    this.t = disabled;
  }
  run(type) {
    if (!type || this.t.read())
      return;
    return super.run(type);
  }
  notify(node, type, flags) {
    if (ActiveTransition && ActiveTransition.r.has(this))
      return ActiveTransition.r.get(this).notify(node, type, flags);
    if (this.t.read()) {
      if (type & LOADING_BIT) {
        if (flags & LOADING_BIT) {
          this.s.add(node);
          type &= ~LOADING_BIT;
        } else if (this.s.delete(node))
          type &= ~LOADING_BIT;
      }
      if (type & ERROR_BIT) {
        if (flags & ERROR_BIT) {
          this.V.add(node);
          type &= ~ERROR_BIT;
        } else if (this.V.delete(node))
          type &= ~ERROR_BIT;
      }
    }
    return type ? super.notify(node, type, flags) : true;
  }
  merge(queue) {
    queue.s.forEach((n) => this.notify(n, LOADING_BIT, LOADING_BIT));
    queue.V.forEach((n) => this.notify(n, ERROR_BIT, ERROR_BIT));
    super.merge(queue);
  }
};
var CollectionQueue = class extends Queue {
  J;
  d = /* @__PURE__ */ new Set();
  t = new Computation(false, null, { pureWrite: true });
  constructor(type) {
    super();
    this.J = type;
  }
  run(type) {
    if (!type || this.t.read())
      return;
    return super.run(type);
  }
  notify(node, type, flags) {
    if (ActiveTransition && ActiveTransition.r.has(this))
      return ActiveTransition.r.get(this).notify(node, type, flags);
    if (!(type & this.J))
      return super.notify(node, type, flags);
    if (flags & this.J) {
      this.d.add(node);
      if (this.d.size === 1)
        this.t.write(true);
    } else if (this.d.size > 0) {
      this.d.delete(node);
      if (this.d.size === 0)
        this.t.write(false);
    }
    type &= ~this.J;
    return type ? super.notify(node, type, flags) : true;
  }
  merge(queue) {
    queue.d.forEach((n) => this.notify(n, this.J, this.J));
    super.merge(queue);
  }
};
function createBoundary(fn, condition) {
  const owner = new Owner();
  const queue = new ConditionalQueue(
    new Computation(void 0, () => condition() === "hidden" /* HIDDEN */)
  );
  const tree = createBoundChildren(owner, fn, queue, 0);
  new EagerComputation(void 0, () => {
    const disabled = queue.t.read();
    tree.U = disabled ? ERROR_BIT | LOADING_BIT : 0;
    if (!disabled) {
      queue.s.forEach((node) => queue.notify(node, LOADING_BIT, LOADING_BIT));
      queue.V.forEach((node) => queue.notify(node, ERROR_BIT, ERROR_BIT));
      queue.s.clear();
      queue.V.clear();
    }
  });
  return () => queue.t.read() ? void 0 : tree.read();
}
function createCollectionBoundary(type, fn, fallback) {
  const owner = new Owner();
  const queue = new CollectionQueue(type);
  const tree = createBoundChildren(owner, fn, queue, type);
  const decision = new Computation(void 0, () => {
    if (!queue.t.read()) {
      const resolved = tree.read();
      if (!untrack(() => queue.t.read()))
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
  return createCollectionBoundary(ERROR_BIT, fn, (queue) => {
    let node = getTransitionSource(queue.d.values().next().value);
    return fallback(node.L, () => {
      incrementClock();
      for (let node2 of queue.d) {
        if (ActiveTransition && !node2.e)
          node2 = cloneGraph(node2);
        node2.c = STATE_DIRTY;
        getQueue(node2).enqueue(node2.C, node2.F.bind(node2));
      }
    });
  });
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

export { $PROXY, $TARGET, $TRACK, Computation, ContextNotFoundError, NoOwnerError, NotReadyError, Owner, Queue, SUPPORTS_PROXY, createAsync, createBoundary, createContext, createEffect, createErrorBoundary, createMemo, createOptimistic, createProjection, createRenderEffect, createRoot, createSignal, createStore, createSuspense, deep, flatten, flush, getContext, getObserver, getOwner, hasContext, hasUpdated, isEqual, isPending, isWrappable, latest, mapArray, merge, omit, onCleanup, reconcile, repeat, resolve, runWithObserver, runWithOwner, setContext, snapshot, transition, untrack, useTransition };
